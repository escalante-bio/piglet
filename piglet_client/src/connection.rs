use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot},
};

pub struct Connection {
    protocols: Arc<Mutex<HashMap<u8, mpsc::Sender<Bytes>>>>,
    stop_tx: oneshot::Sender<()>,
    task: tokio::task::JoinHandle<()>,
    version: u8,
    writer: OwnedWriteHalf,
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

    pub fn unregister_protocol(&self, id: u8) -> () {
        let mut protocols = self.protocols.lock().unwrap();
        protocols.remove(&id);
    }

    pub fn write(&self, protocol: u8, bytes: Bytes) -> Result<(), anyhow::Error> {
        let mut framed = BytesMut::new();
        framed.put_u16_le(1 + 1 + 1 + 1 + bytes.remaining() as u16);
        framed.put_u8(protocol);
        framed.put_u8(self.version);
        framed.put_u8(0);
        framed.put_u8(0);
        framed.put(bytes);
        let copy = framed.freeze();
        self.writer.try_write(&copy)?;
        Ok(())
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
                p[&buffer[2]].clone()
            };
            // Remove the framing
            let data = Bytes::from(buffer[6..buffer_offset].to_vec());
            tx.send(data).await?;
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

async fn initialize(raw: &Connection) -> Result<u16, anyhow::Error> {
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

    initialize_rx.close();
    raw.unregister_protocol(protocol);
    Ok(client_id)
}

pub async fn connect<A: ToSocketAddrs>(
    addr: A,
    version: u8,
) -> Result<ConnectionDetails, anyhow::Error> {
    let stream = TcpStream::connect(addr).await?;
    let (stop_tx, stop_rx) = oneshot::channel();
    let (reader, writer) = stream.into_split();
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
        writer,
    };
    let client_id = initialize(&connection).await?;
    Ok(ConnectionDetails {
        connection,
        client_id,
    })
}
