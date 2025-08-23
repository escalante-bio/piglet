use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelConfiguration;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelType;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::Rail;

use crate::traits::MVec;
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use piglet_client::{
    client::{Error, Error::ConnectionError, RobotClient, with_context},
    object_address::ObjectAddress,
    values::{NetworkResult, PigletCodec},
};
use std::sync::Arc;

pub struct NimbusCoreConfiguration {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreConfiguration {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 384,
            },
            robot: robot.clone(),
        }
    }

    pub async fn is_configuration_saved(&self) -> Result</* saved= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.IsConfigurationSaved()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let saved = bool::deserialize(&mut stream)?;
        Ok(saved)
    }

    pub async fn invalidate_configuration(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.InvalidateConfiguration()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_x_offset(&self, offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || {
                let parameters = vec![format!("  offset: {:?}", offset)];
                format!(
                    "in call to NimbusCoreConfiguration.SetXOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_offset(&self) -> Result</* offset= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 4, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetXOffset()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let offset = i32::deserialize(&mut stream)?;
        Ok(offset)
    }

    pub async fn set_x_limits(
        &self,

        negative_limit: i32,
        positive_limit: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        negative_limit.serialize(&mut args);
        positive_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  negative_limit: {:?}", negative_limit),
                    format!("  positive_limit: {:?}", positive_limit),
                ];
                format!(
                    "in call to NimbusCoreConfiguration.SetXLimits(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_limits(&self) -> Result<GetXLimitsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 6, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetXLimits()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let negative_limit = i32::deserialize(&mut stream)?;
        let positive_limit = i32::deserialize(&mut stream)?;
        Ok(GetXLimitsReply {
            negative_limit,
            positive_limit,
        })
    }

    pub async fn set_x_hazard_limits(
        &self,

        negative_limit: i32,
        positive_limit: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        negative_limit.serialize(&mut args);
        positive_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  negative_limit: {:?}", negative_limit),
                    format!("  positive_limit: {:?}", positive_limit),
                ];
                format!(
                    "in call to NimbusCoreConfiguration.SetXHazardLimits(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_hazard_limits(&self) -> Result<GetXHazardLimitsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 8, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetXHazardLimits()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let negative_limit = i32::deserialize(&mut stream)?;
        let positive_limit = i32::deserialize(&mut stream)?;
        Ok(GetXHazardLimitsReply {
            negative_limit,
            positive_limit,
        })
    }

    pub async fn set_x_max_velocity(&self, velocity: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        velocity.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 9, args.freeze()).await,
            || {
                let parameters = vec![format!("  velocity: {:?}", velocity)];
                format!(
                    "in call to NimbusCoreConfiguration.SetXMaxVelocity(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_max_velocity(&self) -> Result</* velocity= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 10, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetXMaxVelocity()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let velocity = u32::deserialize(&mut stream)?;
        Ok(velocity)
    }

    pub async fn set_y_offset(&self, offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 11, args.freeze()).await,
            || {
                let parameters = vec![format!("  offset: {:?}", offset)];
                format!(
                    "in call to NimbusCoreConfiguration.SetYOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_y_offset(&self) -> Result</* offset= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 12, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetYOffset()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let offset = i32::deserialize(&mut stream)?;
        Ok(offset)
    }

    pub async fn set_y_limits(
        &self,

        negative_limit: i32,
        positive_limit: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        negative_limit.serialize(&mut args);
        positive_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 13, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  negative_limit: {:?}", negative_limit),
                    format!("  positive_limit: {:?}", positive_limit),
                ];
                format!(
                    "in call to NimbusCoreConfiguration.SetYLimits(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_y_limits(&self) -> Result<GetYLimitsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 14, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetYLimits()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let negative_limit = i32::deserialize(&mut stream)?;
        let positive_limit = i32::deserialize(&mut stream)?;
        Ok(GetYLimitsReply {
            negative_limit,
            positive_limit,
        })
    }

    pub async fn set_dual_rail_gantry(&self, dual: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        dual.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 15, args.freeze()).await,
            || {
                let parameters = vec![format!("  dual: {:?}", dual)];
                format!(
                    "in call to NimbusCoreConfiguration.SetDualRailGantry(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_dual_rail_gantry(&self) -> Result</* dual= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 16, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.GetDualRailGantry()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let dual = bool::deserialize(&mut stream)?;
        Ok(dual)
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreConfiguration.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreConfiguration.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreConfiguration.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreConfiguration.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreConfiguration.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreConfiguration.StructInfo(\n{}\n)",
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

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetXLimitsReply {
    negative_limit: i32,
    positive_limit: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetXHazardLimitsReply {
    negative_limit: i32,
    positive_limit: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetYLimitsReply {
    negative_limit: i32,
    positive_limit: i32,
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
