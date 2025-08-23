use crate::client::Error::{CallError, ConnectionError};
use crate::connection::{Connection, ConnectionDetails, connect};
use crate::object_address::ObjectAddress;
use crate::values::{ErrorCode, PigletDeserialize};
use anyhow::{anyhow, bail};
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
    pub async fn connect<T: ToSocketAddrs>(address: T) -> Result<RobotClient, anyhow::Error> {
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
            let message = String::deserialize(&mut bytes)?;
            let errors = if code == ErrorCode(21) {
                message
                    .split(';')
                    .into_iter()
                    .map(|m| parse_robot_error(m))
                    .collect::<anyhow::Result<Vec<_>>>()
                    .map_err(|e| ConnectionError(e))?
            } else {
                vec![parse_robot_error(&message).map_err(|e| ConnectionError(e))?]
            };
            Err(CallError {
                context: None,
                errors,
                source: destination.clone(),
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

pub fn with_context<V, C: FnOnce() -> String>(
    result: Result<V, Error>,
    context_generator: C,
) -> Result<V, Error> {
    result.map_err(|err| {
        let ctx = context_generator();
        match err {
            CallError {
                context,
                errors,
                source,
            } => CallError {
                context: Some(match context {
                    Some(original) => format!("{}\n\ncaused by: {}", ctx, original),
                    None => ctx,
                }),
                errors,
                source,
            },
            ConnectionError(e) => ConnectionError(e.context(ctx)),
        }
    })
}

#[derive(Clone, Debug)]
pub struct RobotError {
    code: ErrorCode,
    source: ObjectAddress,
}

impl std::fmt::Display for RobotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Call to {} failed with code {:?}",
            self.source, self.code
        )
    }
}

#[derive(Debug)]
pub enum Error {
    CallError {
        context: Option<String>,
        errors: Vec<RobotError>,
        source: ObjectAddress,
    },
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
            CallError {
                context,
                errors,
                source,
            } => {
                if errors.len() == 1 {
                    write!(f, "{}", errors.get(0).unwrap())?;
                } else {
                    write!(f, "Call to {} failed with multiple errors:", source)?;
                    for error in errors {
                        write!(f, "\n - {}", error)?;
                    }
                }
                if let Some(c) = context {
                    write!(f, "\n\ncontext: {}", c)?;
                }
                Ok(())
            }
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

fn parse_robot_error(message: &str) -> anyhow::Result<RobotError> {
    let (raw_source, rest) = message
        .split_once(':')
        .ok_or_else(|| anyhow!("Unable to parse error {}", message))?;
    let source_parts: Vec<u16> = raw_source
        .split('.')
        .map(|s| u16::from_str_radix(&s[2..], 16))
        .collect::<Result<Vec<u16>, _>>()?;
    if source_parts.len() != 3 {
        bail!(
            "Unable to parse the robot error source. Original: {}",
            message
        );
    }
    let [module_id, node_id, object_id] = source_parts.try_into().unwrap();
    let source = ObjectAddress {
        module_id,
        node_id,
        object_id,
    };
    let detail_parts: Vec<u16> = rest
        .split(',')
        .map(|s| u16::from_str_radix(&s[2..], 16))
        .collect::<Result<Vec<u16>, _>>()?;
    if detail_parts.len() != 3 {
        bail!(
            "Unable to parse the robot error details. Original: {}",
            message
        );
    }
    let [_unknown, _call_type_id, code] = detail_parts.try_into().unwrap();
    Ok(RobotError {
        source,
        code: ErrorCode(code),
    })
}
