use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelConfiguration;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::ChannelType;
use crate::nimbus_hd_1_0::nimbus_core_global_objects::Rail;

use crate::traits::{MSlice, MVec};
use anyhow::anyhow;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use piglet_client::{
    client::{Error, Error::ConnectionError, RobotClient, with_context},
    object_address::ObjectAddress,
    values::{NetworkResult, PigletCodec, PigletDeserialize, PigletSerialize},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct NimbusCoreXyCoord {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreXyCoord {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 261,
            },
            robot: robot.clone(),
        }
    }

    pub async fn initialize_xy(&self, tips_used: impl AsRef<[u16]>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 1, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used.as_ref())];
                format!(
                    "in call to NimbusCoreXyCoord.InitializeXY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_xy_absolute(
        &self,

        tips_used: impl AsRef<[u16]>,
        x_position: i32,
        y_position: impl AsRef<[i32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used.as_ref()),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position.as_ref()),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXYAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_xy_absolute_plate(
        &self,

        tips_used: impl AsRef<[u16]>,
        gripper_tips_used: impl AsRef<[u16]>,
        x_position: i32,
        x_acceleration: u32,
        y_position: impl AsRef<[i32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        gripper_tips_used.as_ref().serialize(&mut args);
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_position.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used.as_ref()),
                    format!("  gripper_tips_used: {:?}", gripper_tips_used.as_ref()),
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_position: {:?}", y_position.as_ref()),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXYAbsolutePlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_x(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.InitializeX()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_x_absolute(&self, x_position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_position: {:?}", x_position)];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_x_relative_1(&self, x_distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_distance: {:?}", x_distance)];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXRelative_1(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_x_absolute_speed(
        &self,

        x_position: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  velocity: {:?}", velocity),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXAbsoluteSpeed(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_x_calibration(&self, x_offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_offset: {:?}", x_offset)];
                format!(
                    "in call to NimbusCoreXyCoord.SetXCalibration(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_calibration(&self) -> Result<GetXCalibrationReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 9, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetXCalibration()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        Ok(GetXCalibrationReply { x_offset })
    }

    pub async fn set_x_indication_enable(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 10, args.freeze()).await,
            || {
                let parameters = vec![format!("  enable: {:?}", enable)];
                format!(
                    "in call to NimbusCoreXyCoord.SetXIndicationEnable(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_indication_enable(&self) -> Result</* enable= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 11, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetXIndicationEnable()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = bool::deserialize(&mut stream)?;
        Ok(enable)
    }

    pub async fn is_x_initialized(&self) -> Result<IsXInitializedReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 12, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.IsXInitialized()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(IsXInitializedReply { initialized })
    }

    pub async fn get_desired_position(&self) -> Result<GetDesiredPositionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 13, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetDesiredPosition()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_position = i32::deserialize(&mut stream)?;
        let y_position = Vec::<i32>::deserialize(&mut stream)?;
        Ok(GetDesiredPositionReply {
            x_position,
            y_position,
        })
    }

    pub async fn get_settling_parameters(&self) -> Result<GetSettlingParametersReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 14, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetSettlingParameters()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let time = u16::deserialize(&mut stream)?;
        let time_limit = u16::deserialize(&mut stream)?;
        Ok(GetSettlingParametersReply { time, time_limit })
    }

    pub async fn set_settling_parameters(&self, time: u16, time_limit: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        time.serialize(&mut args);
        time_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 15, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  time: {:?}", time),
                    format!("  time_limit: {:?}", time_limit),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.SetSettlingParameters(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn on_the_fly_dispense(
        &self,

        tips_used: impl AsRef<[u16]>,
        x_position: i32,
        acceleration: u32,
        velocity: u32,
        times: impl AsRef<[u32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        x_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        times.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 16, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used.as_ref()),
                    format!("  x_position: {:?}", x_position),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  velocity: {:?}", velocity),
                    format!("  times: {:?}", times.as_ref()),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.OnTheFlyDispense(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_core_gripper_max_y_velocity(&self, y_velocity: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        y_velocity.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 17, args.freeze()).await,
            || {
                let parameters = vec![format!("  y_velocity: {:?}", y_velocity)];
                format!(
                    "in call to NimbusCoreXyCoord.SetCoreGripperMaxYVelocity(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_core_gripper_max_y_velocity(&self) -> Result</* y_velocity= */ u16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 18, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetCoreGripperMaxYVelocity()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_velocity = u16::deserialize(&mut stream)?;
        Ok(y_velocity)
    }

    pub async fn move_x_relative_2(
        &self,

        x_distance: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_distance.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 19, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_distance: {:?}", x_distance),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  velocity: {:?}", velocity),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.MoveXRelative_2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_y_relative(
        &self,

        tips_used: impl AsRef<[u16]>,
        y_distance: impl AsRef<[i32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        y_distance.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 20, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used.as_ref()),
                    format!("  y_distance: {:?}", y_distance.as_ref()),
                ];
                format!(
                    "in call to NimbusCoreXyCoord.MoveYRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_x_velocity_scale(&self, scale: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        scale.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 21, args.freeze()).await,
            || {
                let parameters = vec![format!("  scale: {:?}", scale)];
                format!(
                    "in call to NimbusCoreXyCoord.SetXVelocityScale(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_velocity_scale(&self) -> Result</* scale= */ u16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 22, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.GetXVelocityScale()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let scale = u16::deserialize(&mut stream)?;
        Ok(scale)
    }

    pub async fn unlock(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 23, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.Unlock()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreXyCoord.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreXyCoord.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreXyCoord.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreXyCoord.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreXyCoord.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreXyCoord.StructInfo(\n{}\n)",
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
pub struct GetXCalibrationReply {
    pub x_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct IsXInitializedReply {
    pub initialized: bool,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetDesiredPositionReply {
    pub x_position: i32,
    pub y_position: Vec<i32>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetSettlingParametersReply {
    pub time: u16,
    pub time_limit: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    pub name: String,
    pub version: String,
    pub methods: u32,
    pub subobjects: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct MethodInfoReply {
    pub interfaceid: u8,
    pub action: u8,
    pub actionid: u16,
    pub name: String,
    pub parametertypes: String,
    pub parameternames: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    pub module_id: u16,
    pub node_id: u16,
    pub object_id: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct InterfaceDescriptorsReply {
    pub interface_ids: Vec<u8>,
    pub interface_descriptors: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    pub enumeration_names: Vec<String>,
    pub number_enumeration_values: Vec<u32>,
    pub enumeration_values: Vec<i32>,
    pub enumeration_value_descriptions: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct StructInfoReply {
    pub struct_names: Vec<String>,
    pub number_structure_elements: Vec<u32>,
    pub structure_element_types: Vec<u8>,
    pub structure_element_descriptions: Vec<String>,
}
