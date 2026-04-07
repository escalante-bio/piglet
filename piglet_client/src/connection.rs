use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{
    io::AsyncWriteExt,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot},
};

pub struct Connection {
    protocols: Arc<Mutex<HashMap<u8, mpsc::Sender<Bytes>>>>,
    stop_tx: oneshot::Sender<()>,
    task: tokio::task::JoinHandle<()>,
    version: u8,
    write_tx: mpsc::Sender<Bytes>,
}

impl Connection {
    pub async fn close(self) -> Result<(), anyhow::Error> {
        self.stop_tx
            .send(())
            .or_else(|_| Err(anyhow::anyhow!("Failed to send stop signal")))?;
        self.task.await?;
        Ok(())
    }

    pub fn register_protocol(&self, id: u8) -> mpsc::Receiver<Bytes> {
        let (tx, rx) = mpsc::channel(100);
        let mut protocols = self.protocols.lock().unwrap();
        protocols.insert(id, tx);
        rx
    }

    pub fn write(&self, protocol: u8, bytes: Bytes) -> Result<(), anyhow::Error> {
        let frame = frame_message(protocol, self.version, bytes);
        self.write_tx
            .try_send(frame)
            .map_err(|_| anyhow::anyhow!("Write channel closed"))?;
        Ok(())
    }
}

fn frame_message(protocol: u8, version: u8, bytes: Bytes) -> Bytes {
    let mut framed = BytesMut::new();
    framed.put_u16_le(1 + 1 + 1 + 1 + bytes.remaining() as u16);
    framed.put_u8(protocol);
    framed.put_u8(version);
    framed.put_u8(0);
    framed.put_u8(0);
    framed.put(bytes);
    framed.freeze()
}

async fn write_loop(mut rx: mpsc::Receiver<Bytes>, mut writer: OwnedWriteHalf) {
    while let Some(frame) = rx.recv().await {
        if let Err(e) = writer.write_all(&frame).await {
            eprintln!("piglet: write error: {e}");
            break;
        }
    }
}

pub struct ConnectionDetails {
    pub connection: Connection,
    pub client_id: u16,
}

async fn read_loop(
    protocols: Arc<Mutex<HashMap<u8, mpsc::Sender<Bytes>>>>,
    reader: &OwnedReadHalf,
    mut stop_rx: oneshot::Receiver<()>,
) -> Result<(), anyhow::Error> {
    let mut buffer = vec![0u8; 1024];
    let mut buffer_offset = 0;
    let mut awaiting_length = true;
    let mut packet_length = 0;
    let mut running = true;
    while running {
        let read = match reader.try_read(&mut buffer) {
            Ok(count) => count,
            _ => 0,
        };
        buffer_offset += read;

        if awaiting_length {
            if buffer_offset >= 2 {
                awaiting_length = false;
                packet_length = 2 + (buffer[0] as usize) + ((buffer[1] as usize) << 8);
            }
        } else if packet_length == buffer_offset {
            let tx = {
                let p = protocols.lock().unwrap();
                p.get(&buffer[2]).cloned()
            };
            if let Some(tx) = tx {
                // Remove the framing
                let data = Bytes::from(buffer[6..buffer_offset].to_vec());
                tx.send(data).await?;
            } else {
                eprintln!("piglet: no receiver for protocol {}", buffer[2]);
            }
            awaiting_length = true;
            buffer_offset = 0;
        }

        tokio::select! {
            _ = &mut stop_rx => {
                running = false;
            }
            _ = tokio::time::sleep(Duration::from_millis(if read > 0 { 0 } else { 50 })) => {}
        }
    }
    Ok(())
}

async fn initialize(raw: &Connection) -> Result<(u16, mpsc::Receiver<Bytes>), anyhow::Error> {
    let protocol = 7u8;
    let mut initialize_rx = raw.register_protocol(protocol);

    let mut message = BytesMut::new();
    message.put_u8(0); // version
    message.put_u8(0); // message ID
    message.put_u8(3); // count of things in message
    message.put_u8(0); // ?

    // Request a connection ID
    message.put_u8(1);
    message.put_u8(16);
    message.put_u16_le(0);
    message.put_u16_le(0);

    // Type of connection
    message.put_u8(2);
    message.put_u8(16);
    message.put_u16_le(0);
    message.put_u16_le(4369);

    // Set a timeout
    message.put_u8(4);
    message.put_u8(16);
    message.put_u16_le(0);
    message.put_u16_le(300);
    raw.write(/* protocol= */ 7, message.freeze())?;

    let mut response = initialize_rx.recv().await.unwrap();
    let _version = response.get_u8();
    let message_id = response.get_u8();
    if message_id != 0 {
        anyhow::bail!("Expected message ID 0, not {}", message_id);
    }

    let count = response.get_u8();
    let _unknown = response.get_u8();
    let mut client_id = 0;
    for _ in 0..count {
        let parameter = response.get_u8();
        let meta = response.get_u8();
        let code = response.get_u16_le();
        let value = response.get_u16_le();
        if meta != 17 {
            anyhow::bail!("Expected meta 17, not {}", meta);
        }
        if code != 0 {
            anyhow::bail!("Expected code 0, not {}", code);
        }

        if parameter == 1 {
            client_id = value;
        }
    }

    Ok((client_id, initialize_rx))
}

async fn keep_alive(
    write_tx: mpsc::Sender<Bytes>,
    version: u8,
    mut receiver: mpsc::Receiver<Bytes>,
) {
    let mut message_id: u8 = 1;
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    interval.tick().await; // First tick completes immediately
    loop {
        tokio::select! {
            result = receiver.recv() => {
                if result.is_none() {
                    break;
                }
            }
            _ = interval.tick() => {
                let mut message = BytesMut::new();
                message.put_u8(0); // version
                message.put_u8(message_id);
                message.put_u8(1); // fragment count
                message.put_u8(0);

                // Keep Alive fragment
                message.put_u8(3);
                message.put_u8(16);
                message.put_u16_le(0);
                message.put_u16_le(0);

                let frame = frame_message(7, version, message.freeze());
                if write_tx.try_send(frame).is_err() {
                    break;
                }
                message_id = message_id.wrapping_add(1);
            }
        }
    }
}

pub async fn connect<A: ToSocketAddrs>(
    addr: A,
    version: u8,
) -> Result<ConnectionDetails, anyhow::Error> {
    let stream = TcpStream::connect(addr).await?;
    let (stop_tx, stop_rx) = oneshot::channel();
    let (reader, writer) = stream.into_split();
    let (write_tx, write_rx) = mpsc::channel(100);
    tokio::spawn(write_loop(write_rx, writer));
    let protocols = Arc::new(Mutex::new(HashMap::new()));
    let protocols_clone = protocols.clone();
    let task = tokio::spawn(async move {
        read_loop(protocols_clone, &reader, stop_rx).await.unwrap();
    });

    let connection = Connection {
        protocols,
        stop_tx,
        task,
        version,
        write_tx,
    };
    let (client_id, connection_rx) = initialize(&connection).await?;
    tokio::spawn(keep_alive(
        connection.write_tx.clone(),
        version,
        connection_rx,
    ));
    Ok(ConnectionDetails {
        connection,
        client_id,
    })
}
