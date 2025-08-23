use crate::client::{Error, Error::ConnectionError};
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NetworkResult {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ErrorCode(pub u16);

pub trait PigletCodec: Sized {
    const TYPE_ID: u8;
}

pub trait PigletSerialize: PigletCodec {
    fn serialize(&self, stream: &mut BytesMut);
}

pub trait PigletDeserialize: PigletCodec {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error>;
}

impl PigletCodec for i8 {
    const TYPE_ID: u8 = 1;
}

impl PigletSerialize for i8 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID); // type id
        stream.put_u8(0); // flags
        stream.put_u16_le(1); // length
        stream.put_i8(*self);
    }
}

impl PigletDeserialize for i8 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_i8())
    }
}

impl PigletCodec for i16 {
    const TYPE_ID: u8 = 2;
}

impl PigletSerialize for i16 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(2);
        stream.put_i16_le(*self);
    }
}

impl PigletDeserialize for i16 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_i16_le())
    }
}

impl PigletCodec for i32 {
    const TYPE_ID: u8 = 3;
}

impl PigletSerialize for i32 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self);
    }
}

impl PigletDeserialize for i32 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_i32_le())
    }
}

impl PigletCodec for u8 {
    const TYPE_ID: u8 = 4;
}

impl PigletSerialize for u8 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(1);
        stream.put_u8(*self);
    }
}

impl PigletDeserialize for u8 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_u8())
    }
}

impl PigletCodec for u16 {
    const TYPE_ID: u8 = 5;
}

impl PigletSerialize for u16 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(2);
        stream.put_u16_le(*self);
    }
}

impl PigletDeserialize for u16 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_u16_le())
    }
}

impl PigletCodec for u32 {
    const TYPE_ID: u8 = 6;
}

impl PigletSerialize for u32 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_u32_le(*self);
    }
}

impl PigletDeserialize for u32 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_u32_le())
    }
}

impl PigletCodec for String {
    const TYPE_ID: u8 = 15;
}

impl PigletSerialize for String {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0); // no extra padding
        let length = (self.as_bytes().len() + 1) as u16; // +1 for null terminator
        stream.put_u16_le(length);
        stream.put_slice(self.as_bytes());
        stream.put_u8(0); // null terminator
    }
}

impl PigletDeserialize for String {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let padding = if flags & 1 == 1 { 2 } else { 1 };
        bytes.truncate(bytes.len() - (if bytes.len() > 0 { padding } else { 0 }));
        String::from_utf8(bytes.to_vec()).map_err(|e| ConnectionError(e.into()))
    }
}

impl PigletCodec for bool {
    const TYPE_ID: u8 = 23;
}

impl PigletSerialize for bool {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(1);
        stream.put_u8(*self as u8);
    }
}

impl PigletDeserialize for bool {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(bytes.get_u8() == 1)
    }
}

impl PigletCodec for &[u8] {
    const TYPE_ID: u8 = 22;
}

impl PigletCodec for Vec<u8> {
    const TYPE_ID: u8 = 22;
}

impl PigletDeserialize for Vec<u8> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let bytes = stream.copy_to_bytes(length);

        let mut arr = bytes.to_vec();
        let padding = if flags & 1 == 1 { 1 } else { 0 };
        arr.truncate(arr.len() - (if arr.len() > 0 { padding } else { 0 }));
        Ok(arr)
    }
}

impl PigletSerialize for &[u8] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = self.len() as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0); // no null termination
        stream.put_u16_le(length);
        stream.put_slice(self);
    }
}

impl PigletSerialize for Vec<u8> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for &[i16] {
    const TYPE_ID: u8 = 25;
}

impl PigletCodec for Vec<i16> {
    const TYPE_ID: u8 = 25;
}

impl PigletDeserialize for Vec<i16> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let mut arr = Vec::with_capacity(bytes.len() / 2);
        for _ in 0..(bytes.len() / 2) {
            arr.push(bytes.get_i16_le());
        }
        Ok(arr)
    }
}

impl PigletSerialize for &[i16] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = (self.len() * 2) as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length);
        for &item in self.as_ref() {
            stream.put_i16_le(item);
        }
    }
}

impl PigletSerialize for Vec<i16> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for &[u16] {
    const TYPE_ID: u8 = 26;
}

impl PigletCodec for Vec<u16> {
    const TYPE_ID: u8 = 26;
}

impl PigletDeserialize for Vec<u16> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let mut arr = Vec::with_capacity(bytes.len() / 2);
        for _ in 0..(bytes.len() / 2) {
            arr.push(bytes.get_u16_le());
        }
        Ok(arr)
    }
}

impl PigletSerialize for &[u16] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = (self.len() * 2) as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length);
        for &item in self.as_ref() {
            stream.put_u16_le(item);
        }
    }
}

impl PigletSerialize for Vec<u16> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for &[i32] {
    const TYPE_ID: u8 = 27;
}

impl PigletCodec for Vec<i32> {
    const TYPE_ID: u8 = 27;
}

impl PigletDeserialize for Vec<i32> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let mut arr = Vec::with_capacity(bytes.len() / 4);
        for _ in 0..(bytes.len() / 4) {
            arr.push(bytes.get_i32_le());
        }
        Ok(arr)
    }
}

impl PigletSerialize for &[i32] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = (self.len() * 4) as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length);
        for &item in self.as_ref() {
            stream.put_i32_le(item);
        }
    }
}

impl PigletSerialize for Vec<i32> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for &[u32] {
    const TYPE_ID: u8 = 28;
}

impl PigletCodec for Vec<u32> {
    const TYPE_ID: u8 = 28;
}

impl PigletDeserialize for Vec<u32> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let mut arr = Vec::with_capacity(bytes.len() / 4);
        for _ in 0..(bytes.len() / 4) {
            arr.push(bytes.get_u32_le());
        }
        Ok(arr)
    }
}

impl PigletSerialize for &[u32] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = (self.len() * 4) as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length);
        for &item in self.as_ref() {
            stream.put_u32_le(item);
        }
    }
}

impl PigletSerialize for Vec<u32> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for &[bool] {
    const TYPE_ID: u8 = 29;
}

impl PigletCodec for Vec<bool> {
    const TYPE_ID: u8 = 29;
}

impl PigletDeserialize for Vec<bool> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);

        let mut arr = Vec::with_capacity(bytes.len());
        for _ in 0..bytes.len() {
            arr.push(bytes.get_u8() == 1);
        }
        let padding = if flags & 1 == 1 { 1 } else { 0 };
        arr.truncate(arr.len() - (if arr.len() > 0 { padding } else { 0 }));
        Ok(arr)
    }
}

impl PigletSerialize for &[bool] {
    fn serialize(&self, stream: &mut BytesMut) {
        let length = self.len() as u16;
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length);
        for &item in self.as_ref() {
            stream.put_u8(item as u8);
        }
    }
}

impl PigletSerialize for Vec<bool> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for NetworkResult {
    const TYPE_ID: u8 = 30;
}

impl PigletSerialize for NetworkResult {
    fn serialize(&self, _stream: &mut BytesMut) {
        todo!("literally no idea");
    }
}

impl PigletDeserialize for NetworkResult {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let bytes = stream.copy_to_bytes(length);
        println!("got a network result {:?}", bytes);
        Ok(NetworkResult {})
    }
}

impl PigletCodec for ErrorCode {
    const TYPE_ID: u8 = 33;
}

impl PigletSerialize for ErrorCode {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(2); // u16 is 2 bytes
        stream.put_u16_le(self.0);
    }
}

impl PigletDeserialize for ErrorCode {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let mut bytes = stream.copy_to_bytes(length);
        Ok(Self(bytes.get_u16_le()))
    }
}

impl PigletCodec for &[String] {
    const TYPE_ID: u8 = 34;
}

impl PigletCodec for Vec<String> {
    const TYPE_ID: u8 = 34;
}

impl PigletDeserialize for Vec<String> {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let flags = stream.get_u8();
        let length = stream.get_u16_le() as usize;
        let bytes = stream.copy_to_bytes(length);

        let mut raw = bytes.to_vec();
        let padding = if flags & 1 == 1 { 2 } else { 1 };
        raw.truncate(bytes.len() - (if bytes.len() > 0 { padding } else { 0 }));
        Ok(if raw.len() > 0 {
            raw.split(|v| *v == b'\0')
                .map(|v| String::from_utf8(v.to_vec()).unwrap())
                .collect()
        } else {
            Vec::new()
        })
    }
}

impl PigletSerialize for &[String] {
    fn serialize(&self, stream: &mut BytesMut) {
        let mut length = 0;
        for item in self.iter() {
            length += item.len() + 1; // +1 for null terminator
        }
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(length as u16);
        for item in self.iter() {
            stream.put_slice(item.as_bytes());
            stream.put_u8(0); // null terminator
        }
    }
}

impl PigletSerialize for Vec<String> {
    fn serialize(&self, stream: &mut BytesMut) {
        self.as_slice().serialize(stream)
    }
}

impl PigletCodec for f32 {
    const TYPE_ID: u8 = 40;
}

impl PigletSerialize for f32 {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID); // type id
        stream.put_u8(0); // flags
        stream.put_u16_le(4);
        stream.put_f32_le(*self);
    }
}

impl PigletDeserialize for f32 {
    fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {
        let type_id = stream.get_u8();
        if Self::TYPE_ID != type_id {
            return Err(ConnectionError(anyhow!(
                "Expected {} but got {}",
                Self::TYPE_ID,
                type_id
            )));
        }
        let _flags = stream.get_u8();
        let length = stream.get_u16_le() as usize; // Read length
        let mut bytes = stream.copy_to_bytes(length); // Use length
        Ok(bytes.get_f32_le())
    }
}
