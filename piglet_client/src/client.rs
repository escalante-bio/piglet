use crate::client::Error::{CallError, ConnectionError};
use crate::connection::{Connection, ConnectionDetails, connect};
use crate::object_address::ObjectAddress;
use crate::values::{ErrorCode, PigletCodec};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot},
};

pub struct RobotClient {
    channels: Arc<Mutex<HashMap<ObjectAddress, Channel>>>,
    client_address: ObjectAddress,
    connection: Connection,
    pub globals: Vec<ObjectAddress>,
    pub objects: Vec<ObjectAddress>,
    stop_tx: oneshot::Sender<()>,
    task: tokio::task::JoinHandle<()>,
}

impl RobotClient {
    pub async fn connect<T: ToSocketAddrs>(address: &T) -> Result<RobotClient, anyhow::Error> {
        let ConnectionDetails {
            connection,
            client_id,
        } = connect(&address, 48).await?;
        let receiver = connection.register_protocol(6);
        let (stop_tx, stop_rx) = oneshot::channel();
        let channels = Arc::new(Mutex::new(HashMap::new()));
        let channels_clone = channels.clone();
        let task =
            tokio::spawn(
                async move { read_loop(channels_clone, receiver, stop_rx).await.unwrap() },
            );
        let mut robot = RobotClient {
            channels,
            client_address: ObjectAddress {
                module_id: 2,
                node_id: client_id,
                object_id: 65535,
            },
            connection,
            globals: Vec::new(),
            objects: Vec::new(),
            stop_tx,
            task,
        };
        let (globals, objects) = register(&mut robot).await?;
        robot.globals.extend(globals);
        robot.objects.extend(objects);
        Ok(robot)
    }

    pub async fn close(self) -> Result<(), anyhow::Error> {
        self.stop_tx
            .send(())
            .or_else(|_| Err(anyhow::anyhow!("Failed to send stop signal")))?;
        self.task.await?;
        self.connection.close().await
    }

    pub async fn act(
        &self,
        destination: &ObjectAddress,
        interface_id: u8,
        call_type: u8,
        call_type_id: u16,
        parameters: Bytes,
    ) -> Result<(u8, Bytes), Error> {
        let mut request = BytesMut::new();
        request.put_u8(interface_id);
        request.put_u8(call_type);
        request.put_u16_le(call_type_id);
        request.put_u8(0); // unknown
        request.put_u8(parameters.len() as u8);
        request.put(parameters);

        let response = self
            .request(&destination, 2, call_type, true, request.freeze())
            .await
            .map_err(|e| ConnectionError(e))?;
        let mut bytes = response.bytes;
        let _interface_id = bytes.get_u8();
        let _call_type = bytes.get_u8();
        let _call_type_id = bytes.get_u16_le();
        let _unknown = bytes.get_u8();
        let count = bytes.get_u8();

        if call_type == 0 && response.code == 1 {
            Ok((count, bytes))
        } else if call_type == 3 && response.code == 4 {
            Ok((count, bytes))
        } else {
            let code = ErrorCode::deserialize(&mut bytes)?;
            Err(CallError {
                code,
                remaining: bytes,
            })
        }
    }

    async fn request(
        &self,
        destination: &ObjectAddress,
        protocol: u8,
        call_type: u8,
        require_response: bool,
        bytes: Bytes,
    ) -> Result<Response, anyhow::Error> {
        let (tx, rx) = oneshot::channel();
        let id = {
            let mut channels = self.channels.lock().unwrap();
            let channel = match channels.get_mut(&destination) {
                Some(channel) => channel,
                _ => {
                    let channel = Channel {
                        active: HashMap::new(),
                        id_generator: RequestIdGenerator::default(),
                    };
                    channels.insert(destination.clone(), channel);
                    channels.get_mut(&destination).unwrap()
                }
            };

            let id = channel.id_generator.allocate()?;
            let mut framed = BytesMut::new();
            framed.put(self.client_address.to_bytes());
            framed.put(destination.to_bytes());
            framed.put_u8(id);
            framed.put_u8(0);
            framed.put_u8(protocol);
            framed.put_u8(call_type + (if require_response { 16 } else { 0 }));
            framed.put_u16_le(6 + 6 + 1 + 1 + 1 + 1 + 2 + 2 + 1 + 1 + (bytes.len() as u16));
            framed.put_u16_le(0); // option count
            framed.put_u8(0);
            framed.put_u8(0);
            framed.put(bytes);
            let copy = framed.freeze();
            channel.active.insert(id, tx);
            self.connection.write(/* protocol= */ 6, copy)?;
            id
        };

        let result = rx
            .await
            .or_else(|_| Err(anyhow::anyhow!("Unable to receive response")));

        {
            let mut channels = self.channels.lock().unwrap();
            let channel = channels.get_mut(&destination).unwrap();
            channel.id_generator.release(id);
        }

        result
    }
}

#[derive(Debug)]
pub enum Error {
    CallError { code: ErrorCode, remaining: Bytes },
    ConnectionError(anyhow::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CallError { code, remaining } => write!(
                f,
                "Call failed with code {:?}, remaining bytes: {:?}",
                code, remaining
            ),
            ConnectionError(e) => write!(f, "{}", e.to_string()),
        }
    }
}

struct Channel {
    active: HashMap<u8, oneshot::Sender<Response>>,
    id_generator: RequestIdGenerator,
}

#[derive(Debug, Default)]
struct RequestIdGenerator {
    i: u8,
    active: HashSet<u8>,
}

impl RequestIdGenerator {
    fn allocate(&mut self) -> Result<u8, anyhow::Error> {
        let start = self.i;
        loop {
            let v = self.i;
            self.i = self.i.wrapping_add(1);

            if !self.active.contains(&v) {
                self.active.insert(v);
                return Ok(v);
            } else if self.i == start {
                anyhow::bail!("Ran out of available message IDs");
            }
        }
    }

    fn release(&mut self, v: u8) {
        self.active.remove(&v);
    }
}

#[derive(Debug)]
struct Response {
    protocol: u8,
    code: u8,
    bytes: Bytes,
}

#[derive(Debug)]
struct RegistrationResponse {
    roots: Vec<ObjectAddress>,
}

async fn read_loop(
    channels: Arc<Mutex<HashMap<ObjectAddress, Channel>>>,
    mut receiver: mpsc::Receiver<Bytes>,
    mut stop_rx: oneshot::Receiver<()>,
) -> Result<(), anyhow::Error> {
    let mut running = true;
    while running {
        tokio::select! {
            response = receiver.recv() => {
                if let Some(mut bytes) = response {
                    let expected_length = bytes.remaining();
                    let source = ObjectAddress::from_bytes(&mut bytes)?;
                    let _destination = ObjectAddress::from_bytes(&mut bytes)?;
                    let id = bytes.get_u8();
                    let _unknown = bytes.get_u8();
                    let protocol = bytes.get_u8();
                    let code = bytes.get_u8();
                    let length = bytes.get_u16_le();
                    if expected_length != length.into() {
                        anyhow::bail!("Bad length");
                    }

                    let mut options_length = bytes.get_u16_le();
                    while options_length > 0 {
                        let option = bytes.get_u8();
                        let length = bytes.get_u8() as u16;
                        if option == 1 {
                            if length != 8 {
                                anyhow::bail!("Expected length of 8");
                            }
                            let address = ObjectAddress::from_bytes(&mut bytes);
                            let result = bytes.get_u16_le();
                            if result == 516 {
                                anyhow::bail!("Unable to find address {:?}", address);
                            } else {
                                anyhow::bail!("Unknown communication error with {:?}", address);
                            }
                        } else {
                            bytes.advance(length.into());
                        }
                        options_length -= 1 + 1 + length;
                    }

                    let _ = bytes.get_u8(); // unknown
                    let _ = bytes.get_u8(); // unknown

                    let mut c = channels.lock().unwrap();
                    let channel =
                        c.get_mut(&source).ok_or(anyhow::anyhow!("Missing source"))?;
                    let tx = channel.active.remove(&id).ok_or(anyhow::anyhow!("Missing tx"))?;
                    tx
                        .send(Response { protocol, code, bytes })
                        .or_else(|_| anyhow::bail!("Unable to send response"))?;
                } else {
                    running = false;
                }
            }
            _ = &mut stop_rx => {
                running = false;
            }
        }
    }
    Ok(())
}

async fn register(
    robot: &mut RobotClient,
) -> Result<(Vec<ObjectAddress>, Vec<ObjectAddress>), anyhow::Error> {
    // This is supposed to be 0-0-65534 but it seems to work just as well
    let registration = ObjectAddress {
        module_id: 1,
        node_id: 1,
        object_id: 65534,
    };

    let mut register = BytesMut::new();
    register.put_u16_le(0); // call type
    register.put_u16_le(0); // response code
    register.put_u8(0); // unknown
    register.put_u8(0); // unknown
    register.put(robot.client_address.to_bytes());
    register.put(
        (ObjectAddress {
            module_id: 0,
            node_id: 0,
            object_id: 0,
        })
        .to_bytes(),
    );
    register.put_u16_le(0); // command length
    let register_response = robot
        .request(&registration, 3, 3, false, register.freeze())
        .await?;
    if register_response.protocol != 3 {
        anyhow::bail!("Expected protocol 3, not {}", register_response.protocol);
    }
    read_registration(register_response.bytes)?;

    let mut find_objects = BytesMut::new();
    find_objects.put_u16_le(12); // call type
    find_objects.put_u16_le(0); // response code
    find_objects.put_u8(0); // unknown
    find_objects.put_u8(0); // unknown
    find_objects.put(
        (ObjectAddress {
            module_id: 0,
            node_id: 0,
            object_id: 0,
        })
        .to_bytes(),
    );
    find_objects.put(
        (ObjectAddress {
            module_id: 0,
            node_id: 0,
            object_id: 0,
        })
        .to_bytes(),
    );
    find_objects.put_u16_le(4); // command length
    find_objects.put_u8(5); // master
    find_objects.put_u8(2); // length of data
    find_objects.put_u8(2); // protocol
    find_objects.put_u8(1); // request id
    let find_objects_response = robot
        .request(&registration, 3, 3, true, find_objects.freeze())
        .await?;
    if find_objects_response.protocol != 3 {
        anyhow::bail!(
            "Expected protocol 3, not {}",
            find_objects_response.protocol
        );
    }
    let find_objects_roots = read_registration(find_objects_response.bytes)?.roots;

    let mut find_globals = BytesMut::new();
    find_globals.put_u16_le(12); // call type
    find_globals.put_u16_le(0); // response code
    find_globals.put_u8(0); // unknown
    find_globals.put_u8(0); // unknown
    find_globals.put(
        (ObjectAddress {
            module_id: 0,
            node_id: 0,
            object_id: 0,
        })
        .to_bytes(),
    );
    find_globals.put(
        (ObjectAddress {
            module_id: 0,
            node_id: 0,
            object_id: 0,
        })
        .to_bytes(),
    );
    find_globals.put_u16_le(4); // command length
    find_globals.put_u8(5); // master
    find_globals.put_u8(2); // length of data
    find_globals.put_u8(2); // protocol
    find_globals.put_u8(2); // request id
    let find_globals_response = robot
        .request(&registration, 3, 3, true, find_globals.freeze())
        .await?;
    if find_globals_response.protocol != 3 {
        anyhow::bail!(
            "Expected protocol 3, not {}",
            find_globals_response.protocol
        );
    }
    let find_globals_roots = read_registration(find_globals_response.bytes)?.roots;

    Ok((find_globals_roots, find_objects_roots))
}

fn read_registration(mut bytes: Bytes) -> Result<RegistrationResponse, anyhow::Error> {
    let _call_type_code = bytes.get_u16_le();
    let response_code = bytes.get_u16_le();

    if response_code > 0 {
        anyhow::bail!("Request failed with code {}", response_code);
    }

    let _unknown1 = bytes.get_u8();
    let _unknown2 = bytes.get_u8();

    // these seem pretty useless?
    let _destination = ObjectAddress::from_bytes(&mut bytes)?;
    let _source = ObjectAddress::from_bytes(&mut bytes)?;

    let mut roots = Vec::<ObjectAddress>::new();
    let option_count = bytes.get_u16_le();
    for _ in 0..option_count {
        let option = bytes.get_u8();
        let length = bytes.get_u8();

        if option == 6 {
            if length > 0 {
                let pad = bytes.get_u16_le();
                if (pad & 0x8000) != 0 {
                    anyhow::bail!("some padding issue?");
                }
                for _ in 1..(length / 2) {
                    roots.push(ObjectAddress {
                        module_id: 1,
                        node_id: 1,
                        object_id: bytes.get_u16_le(),
                    });
                }
            }
        } else {
            anyhow::bail!("Unknown option {}", option);
        }

        if bytes.remaining() == 0 {
            break;
        }
    }

    Ok(RegistrationResponse { roots })
}
