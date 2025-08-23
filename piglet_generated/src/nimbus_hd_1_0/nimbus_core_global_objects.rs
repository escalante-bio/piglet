use crate::traits::{MSlice, MVec};
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use piglet_client::{
    client::{Error, Error::ConnectionError, RobotClient, with_context},
    object_address::ObjectAddress,
    values::{NetworkResult, PigletCodec, PigletDeserialize, PigletSerialize},
};
use std::sync::Arc;

pub struct NimbusCoreGlobalObjects {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreGlobalObjects {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 48897,
            },
            robot: robot.clone(),
        }
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreGlobalObjects.ObjectInfo()".to_string(),
        )?;

        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let name = String::deserialize(&mut stream)?;
        let version = String::deserialize(&mut stream)?;
        let methods = u32::deserialize(&mut stream)?;
        let subobjects = u16::deserialize(&mut stream)?;
        Ok(ObjectInfoReply {
            name,
            version,
            methods,
            subobjects,
        })
    }

    pub async fn method_info(&self, method: u32) -> Result<MethodInfoReply, Error> {
        let mut args = BytesMut::new();
        method.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 2, args.freeze()).await,
            || {
                let parameters = vec![format!("  method: {:?}", method)];
                format!(
                    "in call to NimbusCoreGlobalObjects.MethodInfo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 6 {
            return Err(ConnectionError(anyhow!("Expected 6 values, not {}", count)));
        }
        let interfaceid = u8::deserialize(&mut stream)?;
        let action = u8::deserialize(&mut stream)?;
        let actionid = u16::deserialize(&mut stream)?;
        let name = String::deserialize(&mut stream)?;
        let parametertypes = String::deserialize(&mut stream)?;
        let parameternames = String::deserialize(&mut stream)?;
        Ok(MethodInfoReply {
            interfaceid,
            action,
            actionid,
            name,
            parametertypes,
            parameternames,
        })
    }

    pub async fn sub_object_info(&self, subobject: u16) -> Result<SubObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        subobject.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 3, args.freeze()).await,
            || {
                let parameters = vec![format!("  subobject: {:?}", subobject)];
                format!(
                    "in call to NimbusCoreGlobalObjects.SubObjectInfo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let module_id = u16::deserialize(&mut stream)?;
        let node_id = u16::deserialize(&mut stream)?;
        let object_id = u16::deserialize(&mut stream)?;
        Ok(SubObjectInfoReply {
            module_id,
            node_id,
            object_id,
        })
    }

    pub async fn interface_descriptors(&self) -> Result<InterfaceDescriptorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 4, args.freeze()).await,
            || "in call to NimbusCoreGlobalObjects.InterfaceDescriptors()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let interface_ids = Vec::<u8>::deserialize(&mut stream)?;
        let interface_descriptors = Vec::<String>::deserialize(&mut stream)?;
        Ok(InterfaceDescriptorsReply {
            interface_ids,
            interface_descriptors,
        })
    }

    pub async fn enum_info(&self, interface_id: u8) -> Result<EnumInfoReply, Error> {
        let mut args = BytesMut::new();
        interface_id.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 5, args.freeze()).await,
            || {
                let parameters = vec![format!("  interface_id: {:?}", interface_id)];
                format!(
                    "in call to NimbusCoreGlobalObjects.EnumInfo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let enumeration_names = Vec::<String>::deserialize(&mut stream)?;
        let number_enumeration_values = Vec::<u32>::deserialize(&mut stream)?;
        let enumeration_values = Vec::<i32>::deserialize(&mut stream)?;
        let enumeration_value_descriptions = Vec::<String>::deserialize(&mut stream)?;
        Ok(EnumInfoReply {
            enumeration_names,
            number_enumeration_values,
            enumeration_values,
            enumeration_value_descriptions,
        })
    }

    pub async fn struct_info(&self, interface_id: u8) -> Result<StructInfoReply, Error> {
        let mut args = BytesMut::new();
        interface_id.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 6, args.freeze()).await,
            || {
                let parameters = vec![format!("  interface_id: {:?}", interface_id)];
                format!(
                    "in call to NimbusCoreGlobalObjects.StructInfo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let struct_names = Vec::<String>::deserialize(&mut stream)?;
        let number_structure_elements = Vec::<u32>::deserialize(&mut stream)?;
        let structure_element_types = Vec::<u8>::deserialize(&mut stream)?;
        let structure_element_descriptions = Vec::<String>::deserialize(&mut stream)?;
        Ok(StructInfoReply {
            struct_names,
            number_structure_elements,
            structure_element_types,
            structure_element_descriptions,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rail {
    Left = 0,
    Right = 1,
}

impl TryFrom<i32> for Rail {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Rail::Left),
            1 => Ok(Rail::Right),

            _ => Err(ConnectionError(anyhow!("Unknown Rail value {}", v))),
        }
    }
}

impl PigletCodec for Rail {
    const TYPE_ID: u8 = 32;
}

impl PigletDeserialize for Rail {
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
        stream.get_i32_le().try_into()
    }
}

impl PigletSerialize for Rail {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self as i32);
    }
}

impl PigletCodec for MSlice<'_, Rail> {
    const TYPE_ID: u8 = 35;
}

impl PigletCodec for MVec<Rail> {
    const TYPE_ID: u8 = 35;
}

impl PigletDeserialize for MVec<Rail> {
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
        let mut arr = Vec::new();
        for i in 0..(length / 4) {
            arr.push(stream.get_i32_le().try_into()?);
        }
        Ok(MVec(arr))
    }
}

impl PigletSerialize for MSlice<'_, Rail> {
    fn serialize(&self, bytes: &mut BytesMut) {
        bytes.put_u8(Self::TYPE_ID);
        bytes.put_u8(0);
        bytes.put_u16_le(4 * self.0.len() as u16);
        for v in self.0.as_ref() {
            bytes.put_i32_le(*v as i32);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChannelType {
    None = 0,
    Channel300ul = 1,
    Channel1000ul = 2,
    Channel5000ul = 3,
}

impl TryFrom<i32> for ChannelType {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ChannelType::None),
            1 => Ok(ChannelType::Channel300ul),
            2 => Ok(ChannelType::Channel1000ul),
            3 => Ok(ChannelType::Channel5000ul),

            _ => Err(ConnectionError(anyhow!("Unknown ChannelType value {}", v))),
        }
    }
}

impl PigletCodec for ChannelType {
    const TYPE_ID: u8 = 32;
}

impl PigletDeserialize for ChannelType {
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
        stream.get_i32_le().try_into()
    }
}

impl PigletSerialize for ChannelType {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self as i32);
    }
}

impl PigletCodec for MSlice<'_, ChannelType> {
    const TYPE_ID: u8 = 35;
}

impl PigletCodec for MVec<ChannelType> {
    const TYPE_ID: u8 = 35;
}

impl PigletDeserialize for MVec<ChannelType> {
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
        let mut arr = Vec::new();
        for i in 0..(length / 4) {
            arr.push(stream.get_i32_le().try_into()?);
        }
        Ok(MVec(arr))
    }
}

impl PigletSerialize for MSlice<'_, ChannelType> {
    fn serialize(&self, bytes: &mut BytesMut) {
        bytes.put_u8(Self::TYPE_ID);
        bytes.put_u8(0);
        bytes.put_u16_le(4 * self.0.len() as u16);
        for v in self.0.as_ref() {
            bytes.put_i32_le(*v as i32);
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelConfiguration {
    pub type_: ChannelType,
    pub rail: Rail,
    pub previous_neighbor_spacing: u16,
    pub next_neighbor_spacing: u16,
    pub can_address: u8,
}

impl PigletCodec for ChannelConfiguration {
    const TYPE_ID: u8 = 30;
}

impl PigletSerialize for ChannelConfiguration {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.type_.serialize(&mut buffer);
        s.rail.serialize(&mut buffer);
        s.previous_neighbor_spacing.serialize(&mut buffer);
        s.next_neighbor_spacing.serialize(&mut buffer);
        s.can_address.serialize(&mut buffer);

        stream.put_u16_le(buffer.len() as u16);
        stream.put(buffer);
    }
}

impl PigletDeserialize for ChannelConfiguration {
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
        Ok(Self {
            type_: ChannelType::deserialize(&mut bytes)?,
            rail: Rail::deserialize(&mut bytes)?,
            previous_neighbor_spacing: u16::deserialize(&mut bytes)?,
            next_neighbor_spacing: u16::deserialize(&mut bytes)?,
            can_address: u8::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MSlice<'_, ChannelConfiguration> {
    const TYPE_ID: u8 = 31;
}

impl PigletCodec for MVec<ChannelConfiguration> {
    const TYPE_ID: u8 = 31;
}

impl PigletSerialize for MSlice<'_, ChannelConfiguration> {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in self.0.as_ref() {
            let mut buffer = BytesMut::new();

            s.type_.serialize(&mut buffer);
            s.rail.serialize(&mut buffer);
            s.previous_neighbor_spacing.serialize(&mut buffer);
            s.next_neighbor_spacing.serialize(&mut buffer);
            s.can_address.serialize(&mut buffer);

            outer.put_u16_le(buffer.len() as u16);
            outer.put(buffer);
        }

        stream.put_u16_le(outer.len() as u16);
        stream.put(outer);
    }
}

impl PigletSerialize for MVec<ChannelConfiguration> {
    fn serialize(&self, stream: &mut BytesMut) {
        MSlice(&self.0).serialize(stream)
    }
}

impl PigletDeserialize for MVec<ChannelConfiguration> {
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
        let mut length = stream.get_u16_le() as usize;
        let mut outer = stream.copy_to_bytes(length);
        let mut arr = Vec::new();
        while length > 0 {
            let mut length = outer.get_u16_le() as usize;
            let mut bytes = outer.copy_to_bytes(length);
            arr.push(ChannelConfiguration {
                type_: ChannelType::deserialize(&mut bytes)?,
                rail: Rail::deserialize(&mut bytes)?,
                previous_neighbor_spacing: u16::deserialize(&mut bytes)?,
                next_neighbor_spacing: u16::deserialize(&mut bytes)?,
                can_address: u8::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MethodInfoReply {
    interfaceid: u8,
    action: u8,
    actionid: u16,
    name: String,
    parametertypes: String,
    parameternames: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct InterfaceDescriptorsReply {
    interface_ids: Vec<u8>,
    interface_descriptors: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    enumeration_names: Vec<String>,
    number_enumeration_values: Vec<u32>,
    enumeration_values: Vec<i32>,
    enumeration_value_descriptions: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}
