use crate::client::{Error, Error::ConnectionError, RobotClient};
use crate::object_address::ObjectAddress;
use crate::values::{PigletDeserialize, PigletSerialize};
use anyhow::anyhow;
use bytes::{Buf, Bytes, BytesMut};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    pub labels: Vec<String>,
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Interface {
    pub label: String,
    pub id: u8,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Method {
    pub call_type: u8,
    pub interface_id: u8,
    pub method_id: u16,
    pub name: String,
    pub parameter_labels: Vec<String>,
    pub parameter_types: Vec<u8>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Object {
    pub address: ObjectAddress,
    pub name: String,
    pub version: String,
    pub method_count: u32,
    pub subobject_count: u16,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Struct {
    pub element_labels: Vec<String>,
    pub element_types: Vec<u8>,
    pub name: String,
}

pub struct DynamicObject {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl DynamicObject {
    pub fn new(address: &ObjectAddress, robot: &Arc<RobotClient>) -> Self {
        Self {
            address: address.clone(),
            robot: robot.clone(),
        }
    }

    pub async fn get_object(&self) -> Result<Object, Error> {
        let (count, mut stream) = self.robot.act(&self.address, 0, 0, 1, Bytes::new()).await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let name = String::deserialize(&mut stream)?;
        let version = String::deserialize(&mut stream)?;
        let method_count = u32::deserialize(&mut stream)?;
        let subobject_count = u16::deserialize(&mut stream)?;
        Ok(Object {
            address: self.address.clone(),
            name,
            version,
            method_count,
            subobject_count,
        })
    }

    pub async fn get_subobject_address(&self, index: u16) -> Result<ObjectAddress, Error> {
        let mut args = BytesMut::new();
        index.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 3, args.freeze())
            .await?;
        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let module_id = u16::deserialize(&mut stream)?;
        let node_id = u16::deserialize(&mut stream)?;
        let object_id = u16::deserialize(&mut stream)?;
        Ok(ObjectAddress {
            module_id,
            node_id,
            object_id,
        })
    }

    pub async fn get_enums(&self, interface_id: u8) -> Result<Vec<Enum>, Error> {
        let mut args = BytesMut::new();
        interface_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 5, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let names = Vec::<String>::deserialize(&mut stream)?;
        let numbers = Vec::<u32>::deserialize(&mut stream)?;
        let enum_values = Vec::<i32>::deserialize(&mut stream)?;
        let descriptions = Vec::<String>::deserialize(&mut stream)?;

        let mut enums = Vec::new();
        let mut offset = 0;
        for i in 0..names.len() {
            let count = numbers[i] as usize;
            enums.push(Enum {
                name: names[i].clone(),
                labels: descriptions[offset..(offset + count)].to_vec(),
                values: enum_values[offset..(offset + count)].to_vec(),
            });
            offset += count;
        }
        Ok(enums)
    }

    pub async fn get_interfaces(&self) -> Result<Vec<Interface>, Error> {
        let (count, mut stream) = self.robot.act(&self.address, 0, 0, 4, Bytes::new()).await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let ids = Vec::<u8>::deserialize(&mut stream)?;
        let descriptions = Vec::<String>::deserialize(&mut stream)?;
        let mut interfaces = Vec::new();
        for i in 0..ids.len() {
            interfaces.push(Interface {
                id: ids[i],
                label: descriptions[i].clone(),
            });
        }
        Ok(interfaces)
    }

    pub async fn get_method(&self, index: u32) -> Result<Method, Error> {
        let mut args = BytesMut::new();
        index.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 2, args.freeze())
            .await?;
        if count != 6 {
            return Err(ConnectionError(anyhow!("Expected 6 values, not {}", count)));
        }
        let interface_id = u8::deserialize(&mut stream)?;
        let call_type = u8::deserialize(&mut stream)?;
        let method_id = u16::deserialize(&mut stream)?;
        let name = String::deserialize(&mut stream)?;
        let parameter_types = String::deserialize(&mut stream)?.as_bytes().to_vec();
        let parameter_labels = if stream.remaining() > 0 {
            let s = String::deserialize(&mut stream)?;
            if s.len() > 0 {
                s.split(",").map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            }
        } else {
            vec![]
        };
        Ok(Method {
            call_type,
            interface_id,
            method_id,
            name,
            parameter_labels,
            parameter_types,
        })
    }

    pub async fn get_structs(&self, interface_id: u8) -> Result<Vec<Struct>, Error> {
        let mut args = BytesMut::new();
        interface_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 6, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let names = Vec::<String>::deserialize(&mut stream)?;
        let numbers = Vec::<u32>::deserialize(&mut stream)?;
        let element_types = Vec::<u8>::deserialize(&mut stream)?;
        let element_labels = Vec::<String>::deserialize(&mut stream)?;

        let mut structs = Vec::new();
        let mut labels_offset = 0;
        let mut types_offset = 0;
        for i in 0..names.len() {
            let labels_count = numbers[i] as usize;
            let mut types_count = 0;
            for _i in 0..labels_count {
                let jump = match element_types[types_offset + types_count] {
                    30 => 3,
                    32 => 3,
                    _ => 1,
                };
                types_count += jump;
            }
            structs.push(Struct {
                name: names[i].clone(),
                element_labels: element_labels[labels_offset..(labels_offset + labels_count)]
                    .to_vec(),
                element_types: element_types[types_offset..(types_offset + types_count)].to_vec(),
            });
            labels_offset += labels_count;
            types_offset += types_count;
        }
        Ok(structs)
    }
}
