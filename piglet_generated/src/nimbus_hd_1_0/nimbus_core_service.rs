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

pub struct NimbusCoreService {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreService {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 48880,
            },
            robot: robot.clone(),
        }
    }

    pub async fn get_channel_technical_info(
        &self,
        channel_number: u16,
    ) -> Result</* channel_info= */ String, Error> {
        let mut args = BytesMut::new();
        channel_number.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 7, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channel_info = String::deserialize(&mut stream)?;
        Ok(channel_info)
    }

    pub async fn get_channel_counters(&self) -> Result<GetChannelCountersReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 6, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_pickup_counter = Vec::<u32>::deserialize(&mut stream)?;
        let tip_eject_counter = Vec::<u32>::deserialize(&mut stream)?;
        let aspirate_counter = Vec::<u32>::deserialize(&mut stream)?;
        let dispense_counter = Vec::<u32>::deserialize(&mut stream)?;
        Ok(GetChannelCountersReply {
            tip_pickup_counter,
            tip_eject_counter,
            aspirate_counter,
            dispense_counter,
        })
    }

    pub async fn get_x_home_sensor(&self) -> Result</* x_home= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 1, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_home = bool::deserialize(&mut stream)?;
        Ok(x_home)
    }

    pub async fn z_servo_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
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

    pub async fn get_channel_home_sensors(&self) -> Result<GetChannelHomeSensorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 2, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let y_home = Vec::<bool>::deserialize(&mut stream)?;
        let z_home = Vec::<bool>::deserialize(&mut stream)?;
        let squeeze_home = Vec::<bool>::deserialize(&mut stream)?;
        let dispense_home = Vec::<bool>::deserialize(&mut stream)?;
        Ok(GetChannelHomeSensorsReply {
            y_home,
            z_home,
            squeeze_home,
            dispense_home,
        })
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

    pub async fn get_positions(&self) -> Result<GetPositionsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 16, args.freeze())
            .await?;
        if count != 10 {
            return Err(ConnectionError(anyhow!(
                "Expected 10 values, not {}",
                count
            )));
        }
        let x_position = i32::deserialize(&mut stream)?;
        let x_encoder_position = i32::deserialize(&mut stream)?;
        let y_position = Vec::<i16>::deserialize(&mut stream)?;
        let y_encoder_position = Vec::<i16>::deserialize(&mut stream)?;
        let z_position = Vec::<i16>::deserialize(&mut stream)?;
        let z_encoder_position = Vec::<i16>::deserialize(&mut stream)?;
        let d_position = Vec::<i16>::deserialize(&mut stream)?;
        let d_encoder_position = Vec::<i16>::deserialize(&mut stream)?;
        let s_position = Vec::<i16>::deserialize(&mut stream)?;
        let s_encoder_position = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetPositionsReply {
            x_position,
            x_encoder_position,
            y_position,
            y_encoder_position,
            z_position,
            z_encoder_position,
            d_position,
            d_encoder_position,
            s_position,
            s_encoder_position,
        })
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

    pub async fn get_channel_configuration(
        &self,
        channel: u16,
        indexes: Vec<i16>,
    ) -> Result</* enabled= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        indexes.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 14, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enabled = Vec::<bool>::deserialize(&mut stream)?;
        Ok(enabled)
    }

    pub async fn x_servo_control(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 10, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_channel_extended_calibration_values(
        &self,
    ) -> Result<GetChannelExtendedCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 5, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_type_adjustment = Vec::<i16>::deserialize(&mut stream)?;
        let pressure_conversion = Vec::<i16>::deserialize(&mut stream)?;
        let aspirate_monitoring = Vec::<i16>::deserialize(&mut stream)?;
        let z_bottom_search = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetChannelExtendedCalibrationValuesReply {
            tip_type_adjustment,
            pressure_conversion,
            aspirate_monitoring,
            z_bottom_search,
        })
    }

    pub async fn get_channel_calibration_values(
        &self,
    ) -> Result<GetChannelCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 4, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let squeeze = Vec::<i16>::deserialize(&mut stream)?;
        let pressure_lld = Vec::<i16>::deserialize(&mut stream)?;
        let clot_detection = Vec::<i16>::deserialize(&mut stream)?;
        let dispense_calibration = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetChannelCalibrationValuesReply {
            squeeze,
            pressure_lld,
            clot_detection,
            dispense_calibration,
        })
    }

    pub async fn get_board_information(&self) -> Result<GetBoardInformationReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 18, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let board_version = u32::deserialize(&mut stream)?;
        let pld_version = u32::deserialize(&mut stream)?;
        Ok(GetBoardInformationReply {
            board_version,
            pld_version,
        })
    }

    pub async fn get_channel_tip_sensors(&self) -> Result</* tip_presence= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 3, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_presence = Vec::<i16>::deserialize(&mut stream)?;
        Ok(tip_presence)
    }

    pub async fn get_channel_types_1(&self) -> Result</* channel_types= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 8, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channel_types = Vec::<i16>::deserialize(&mut stream)?;
        Ok(channel_types)
    }

    pub async fn get_channel_types_2(&self) -> Result</* channel_types= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 8, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channel_types = Vec::<i16>::deserialize(&mut stream)?;
        Ok(channel_types)
    }

    pub async fn x_servo_status(&self) -> Result</* enabled= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 11, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enabled = bool::deserialize(&mut stream)?;
        Ok(enabled)
    }

    pub async fn y_servo_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 12, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_channel_counters(
        &self,
        tips_used: Vec<u16>,
        tip_pickup_counter: Vec<u32>,
        tip_eject_counter: Vec<u32>,
        aspirate_counter: Vec<u32>,
        dispense_counter: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        tip_pickup_counter.serialize(&mut args);
        tip_eject_counter.serialize(&mut args);
        aspirate_counter.serialize(&mut args);
        dispense_counter.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn measure_lld_frequency(
        &self,
        channel: u16,
        seconds: u16,
    ) -> Result</* detects_per_minute= */ u16, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        seconds.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 15, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let detects_per_minute = u16::deserialize(&mut stream)?;
        Ok(detects_per_minute)
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
}

#[derive(Clone, Debug)]
pub struct GetChannelCountersReply {
    tip_pickup_counter: Vec<u32>,
    tip_eject_counter: Vec<u32>,
    aspirate_counter: Vec<u32>,
    dispense_counter: Vec<u32>,
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

#[derive(Clone, Debug)]
pub struct GetChannelHomeSensorsReply {
    y_home: Vec<bool>,
    z_home: Vec<bool>,
    squeeze_home: Vec<bool>,
    dispense_home: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GetPositionsReply {
    x_position: i32,
    x_encoder_position: i32,
    y_position: Vec<i16>,
    y_encoder_position: Vec<i16>,
    z_position: Vec<i16>,
    z_encoder_position: Vec<i16>,
    d_position: Vec<i16>,
    d_encoder_position: Vec<i16>,
    s_position: Vec<i16>,
    s_encoder_position: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
}

#[derive(Clone, Debug)]
pub struct GetChannelExtendedCalibrationValuesReply {
    tip_type_adjustment: Vec<i16>,
    pressure_conversion: Vec<i16>,
    aspirate_monitoring: Vec<i16>,
    z_bottom_search: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetChannelCalibrationValuesReply {
    squeeze: Vec<i16>,
    pressure_lld: Vec<i16>,
    clot_detection: Vec<i16>,
    dispense_calibration: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetBoardInformationReply {
    board_version: u32,
    pld_version: u32,
}

#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
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
