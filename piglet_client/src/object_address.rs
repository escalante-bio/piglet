use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObjectAddress {
    pub module_id: u16,
    pub node_id: u16,
    pub object_id: u16,
}

impl ObjectAddress {
    pub fn from_bytes(bytes: &mut Bytes) -> Result<Self, anyhow::Error> {
        if bytes.len() < 6 {
            anyhow::bail!(
                "Not enough bytes for ObjectAddress: expected 6, got {}",
                bytes.len()
            );
        }
        Ok(ObjectAddress {
            module_id: bytes.get_u16_le(),
            node_id: bytes.get_u16_le(),
            object_id: bytes.get_u16_le(),
        })
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(6);
        buf.put_u16_le(self.module_id);
        buf.put_u16_le(self.node_id);
        buf.put_u16_le(self.object_id);
        buf.freeze()
    }
}

impl std::fmt::Display for ObjectAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.module_id, self.node_id, self.object_id
        )
    }
}
