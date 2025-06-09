use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelConfiguration;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelType;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::Rail;

use crate::traits::MVec;
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use piglet_client::{
    client::{Error, Error::ConnectionError, RobotClient},
    object_address::ObjectAddress,
    values::{NetworkResult, PigletCodec},
};
use std::sync::Arc;

pub struct NimbusCoreEthernet {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreEthernet {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 259,
            },
            robot: robot.clone(),
        }
    }

    pub async fn get_current_host_name(&self) -> Result</* host_name= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 5, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let host_name = String::deserialize(&mut stream)?;
        Ok(host_name)
    }

    pub async fn get_instrument_id(&self) -> Result</* instrument_id= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 18, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let instrument_id = String::deserialize(&mut stream)?;
        Ok(instrument_id)
    }

    pub async fn get_mac_address(&self) -> Result</* mac_address= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 10, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let mac_address = String::deserialize(&mut stream)?;
        Ok(mac_address)
    }

    pub async fn set_instrument_id(&self, instrument_id: String) -> Result<(), Error> {
        let mut args = BytesMut::new();
        instrument_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 17, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_dhcp_enable(&self, enable_dhcp: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable_dhcp.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 1, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_dhcp_timeout(&self) -> Result</* dhcp_timeout= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 12, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let dhcp_timeout = i32::deserialize(&mut stream)?;
        Ok(dhcp_timeout)
    }

    pub async fn get_host_name(&self) -> Result</* host_name= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 4, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let host_name = String::deserialize(&mut stream)?;
        Ok(host_name)
    }

    pub async fn get_dhcp_enable(&self) -> Result</* use_dhcp= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 2, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let use_dhcp = bool::deserialize(&mut stream)?;
        Ok(use_dhcp)
    }

    pub async fn get_current_ip_address(&self) -> Result<GetCurrentIpAddressReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 9, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let ip_address = String::deserialize(&mut stream)?;
        let subnet_mask = String::deserialize(&mut stream)?;
        Ok(GetCurrentIpAddressReply {
            ip_address,
            subnet_mask,
        })
    }

    pub async fn get_comment(&self) -> Result</* comment= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 16, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let comment = String::deserialize(&mut stream)?;
        Ok(comment)
    }

    pub async fn get_instrument_type(&self) -> Result</* instrument_type= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 19, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let instrument_type = String::deserialize(&mut stream)?;
        Ok(instrument_type)
    }

    pub async fn are_stored_settings_valid(&self) -> Result</* valid= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 23, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let valid = bool::deserialize(&mut stream)?;
        Ok(valid)
    }

    pub async fn get_tcp_port(&self) -> Result</* port= */ u16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 24, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let port = u16::deserialize(&mut stream)?;
        Ok(port)
    }

    pub async fn set_udp_port(&self, port: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        port.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 27, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 1, args.freeze())
            .await?;
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

    pub async fn save_settings(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 20, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_udp_port(&self) -> Result</* port= */ u16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 26, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let port = u16::deserialize(&mut stream)?;
        Ok(port)
    }

    pub async fn sub_object_info(&self, subobject: u16) -> Result<SubObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        subobject.serialize(&mut args);
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
        Ok(SubObjectInfoReply {
            module_id,
            node_id,
            object_id,
        })
    }

    pub async fn invalidate_settings(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 22, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_location(&self, location: String) -> Result<(), Error> {
        let mut args = BytesMut::new();
        location.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_static_ip_address(&self) -> Result<GetStaticIpAddressReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 8, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let ip_address = String::deserialize(&mut stream)?;
        let subnet_mask = String::deserialize(&mut stream)?;
        Ok(GetStaticIpAddressReply {
            ip_address,
            subnet_mask,
        })
    }

    pub async fn get_location(&self) -> Result</* location= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 14, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let location = String::deserialize(&mut stream)?;
        Ok(location)
    }

    pub async fn interface_descriptors(&self) -> Result<InterfaceDescriptorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 4, args.freeze())
            .await?;
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
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 5, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let enumeration_names = Vec::<String>::deserialize(&mut stream)?;
        let number_enumeration_values = Vec::<u32>::deserialize(&mut stream)?;
        let enumeration_values = Vec::<i16>::deserialize(&mut stream)?;
        let enumeration_value_descriptions = Vec::<String>::deserialize(&mut stream)?;
        Ok(EnumInfoReply {
            enumeration_names,
            number_enumeration_values,
            enumeration_values,
            enumeration_value_descriptions,
        })
    }

    pub async fn set_host_name(&self, host_name: String) -> Result<(), Error> {
        let mut args = BytesMut::new();
        host_name.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_dhcp_timeout(&self, dhcp_timeout: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        dhcp_timeout.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_comment(&self, comment: String) -> Result<(), Error> {
        let mut args = BytesMut::new();
        comment.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 15, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_static_ip_address(
        &self,
        ip_address: String,
        subnet_mask: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        ip_address.serialize(&mut args);
        subnet_mask.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn struct_info(&self, interface_id: u8) -> Result<StructInfoReply, Error> {
        let mut args = BytesMut::new();
        interface_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 6, args.freeze())
            .await?;
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

    pub async fn get_default_host_name(&self) -> Result</* host_name= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 6, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let host_name = String::deserialize(&mut stream)?;
        Ok(host_name)
    }

    pub async fn set_tcp_port(&self, port: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        port.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 25, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn recall_settings(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 21, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn method_info(&self, method: u32) -> Result<MethodInfoReply, Error> {
        let mut args = BytesMut::new();
        method.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 0, 0, 2, args.freeze())
            .await?;
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
}

#[derive(Clone, Debug)]
pub struct GetCurrentIpAddressReply {
    ip_address: String,
    subnet_mask: String,
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
}

#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
}

#[derive(Clone, Debug)]
pub struct GetStaticIpAddressReply {
    ip_address: String,
    subnet_mask: String,
}

#[derive(Clone, Debug)]
pub struct InterfaceDescriptorsReply {
    interface_ids: Vec<u8>,
    interface_descriptors: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    enumeration_names: Vec<String>,
    number_enumeration_values: Vec<u32>,
    enumeration_values: Vec<i16>,
    enumeration_value_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct MethodInfoReply {
    interfaceid: u8,
    action: u8,
    actionid: u16,
    name: String,
    parametertypes: String,
    parameternames: String,
}
