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

pub struct NimbusCore {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCore {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 48896,
            },
            robot: robot.clone(),
        }
    }

    pub async fn initialize(
        &self,

        x_position: Vec<i32>,
        y_position: Vec<i32>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 1, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_smart(
        &self,

        x_position: Vec<i32>,
        y_position: Vec<i32>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 2, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn park(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn park_button_park(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 4, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn shift_and_scan_rack(
        &self,

        traverse_height: i32,
        min_x_position: i32,
        max_x_position: i32,
        cam_position: i16,
    ) -> Result<ShiftAndScanRackReply, Error> {
        let mut args = BytesMut::new();
        traverse_height.serialize(&mut args);
        min_x_position.serialize(&mut args);
        max_x_position.serialize(&mut args);
        cam_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 5, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_positions = Vec::<i32>::deserialize(&mut stream)?;
        let barcodes = String::deserialize(&mut stream)?;
        Ok(ShiftAndScanRackReply {
            x_positions,
            barcodes,
        })
    }

    pub async fn shift_and_scan_row(
        &self,

        traverse_height: i32,
        x_position: i32,
        cams_used: Vec<u16>,
        cam_positions: Vec<i16>,
    ) -> Result<ShiftAndScanRowReply, Error> {
        let mut args = BytesMut::new();
        traverse_height.serialize(&mut args);
        x_position.serialize(&mut args);
        cams_used.serialize(&mut args);
        cam_positions.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 6, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let barcodes = String::deserialize(&mut stream)?;
        Ok(ShiftAndScanRowReply { barcodes })
    }

    pub async fn method_begin(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn method_end(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 8, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_instrument_error_mode(&self, instrument_error: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        instrument_error.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_instrument_error_mode(&self) -> Result</* instrument_error= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 10, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let instrument_error = bool::deserialize(&mut stream)?;
        Ok(instrument_error)
    }

    pub async fn set_operator_attention_mode(&self, operator_attention: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        operator_attention.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_operator_attention_mode(
        &self,
    ) -> Result</* operator_attention= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 12, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let operator_attention = bool::deserialize(&mut stream)?;
        Ok(operator_attention)
    }

    pub async fn get_xml_compatibility(&self) -> Result</* level= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 13, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let level = String::deserialize(&mut stream)?;
        Ok(level)
    }

    pub async fn is_initialized(&self) -> Result</* initialized= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 14, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(initialized)
    }

    pub async fn get_channel_configuration_1(
        &self,
    ) -> Result<GetChannelConfiguration_1Reply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 15, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let channels = u16::deserialize(&mut stream)?;
        let channel_types = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetChannelConfiguration_1Reply {
            channels,
            channel_types,
        })
    }

    pub async fn get_channel_configuration_2(
        &self,
    ) -> Result<GetChannelConfiguration_2Reply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 15, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let channels = u16::deserialize(&mut stream)?;
        let channel_types = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetChannelConfiguration_2Reply {
            channels,
            channel_types,
        })
    }

    pub async fn preinitialize_smart(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 16, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_plate_gripped(&self) -> Result</* plate_gripped= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 17, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let plate_gripped = bool::deserialize(&mut stream)?;
        Ok(plate_gripped)
    }

    pub async fn is_gripper_present(&self) -> Result</* present= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 18, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let present = bool::deserialize(&mut stream)?;
        Ok(present)
    }

    pub async fn is_shift_and_scan_present(&self) -> Result</* present= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 19, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let present = bool::deserialize(&mut stream)?;
        Ok(present)
    }

    pub async fn is_device_present_1(&self, device_id: i16) -> Result</* present= */ bool, Error> {
        let mut args = BytesMut::new();
        device_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 20, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let present = bool::deserialize(&mut stream)?;
        Ok(present)
    }

    pub async fn is_device_present_2(&self, device_id: i16) -> Result</* present= */ bool, Error> {
        let mut args = BytesMut::new();
        device_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 20, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let present = bool::deserialize(&mut stream)?;
        Ok(present)
    }

    pub async fn set_x_speed_scale(&self, scale: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        scale.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 21, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_speed_scale(&self) -> Result</* scale= */ u16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 22, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let scale = u16::deserialize(&mut stream)?;
        Ok(scale)
    }

    pub async fn get_data_store(&self, handle: i16) -> Result</* data= */ Vec<u8>, Error> {
        let mut args = BytesMut::new();
        handle.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 23, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let data = Vec::<u8>::deserialize(&mut stream)?;
        Ok(data)
    }

    pub async fn set_data_store(&self, handle: i16, data: Vec<u8>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        handle.serialize(&mut args);
        data.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 24, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_shift_and_scan_sensors(&self) -> Result<GetShiftAndScanSensorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 25, args.freeze())
            .await?;
        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let sensor_1 = bool::deserialize(&mut stream)?;
        let sensor_2 = bool::deserialize(&mut stream)?;
        let sensor_3 = bool::deserialize(&mut stream)?;
        Ok(GetShiftAndScanSensorsReply {
            sensor_1,
            sensor_2,
            sensor_3,
        })
    }

    pub async fn reset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 26, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn shift_and_scan_rack_x_speed(
        &self,

        traverse_height: i32,
        min_x_position: i32,
        max_x_position: i32,
        x_speed: u32,
        cam_position: i16,
    ) -> Result<ShiftAndScanRackXSpeedReply, Error> {
        let mut args = BytesMut::new();
        traverse_height.serialize(&mut args);
        min_x_position.serialize(&mut args);
        max_x_position.serialize(&mut args);
        x_speed.serialize(&mut args);
        cam_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 27, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_positions = Vec::<i32>::deserialize(&mut stream)?;
        let barcodes = String::deserialize(&mut stream)?;
        Ok(ShiftAndScanRackXSpeedReply {
            x_positions,
            barcodes,
        })
    }

    pub async fn initialize_roll(
        &self,

        x_position: Vec<i32>,
        y_position: Vec<i32>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
        roll_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        roll_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 28, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_smart_roll(
        &self,

        x_position: Vec<i32>,
        y_position: Vec<i32>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
        roll_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        roll_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 29, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_velocity_limits(&self) -> Result<GetXVelocityLimitsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 32, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let lower_limit = u32::deserialize(&mut stream)?;
        let upper_limit = u32::deserialize(&mut stream)?;
        Ok(GetXVelocityLimitsReply {
            lower_limit,
            upper_limit,
        })
    }

    pub async fn set_x_maximum_velocity(&self, velocity: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 33, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_x_maximum_velocity(&self) -> Result</* velocity= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 34, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let velocity = u32::deserialize(&mut stream)?;
        Ok(velocity)
    }

    pub async fn reset_x_maximum_velocity(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 35, args.freeze())
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
}

#[derive(Clone, Copy, Debug)]
pub enum DeviceId {
    DeviceIdInvalid = 0,
    DeviceIdChannel1 = 1,
    DeviceIdChannel2 = 2,
    DeviceIdChannel3 = 3,
    DeviceIdChannel4 = 4,
    DeviceIdGripper = 5,
    DeviceIdBarcodeReader = 6,
    DeviceIdShiftAndScan = 7,
    DeviceIdBdPuncture = 8,
    DeviceIdSiemensCarousel = 9,
    DeviceIdDoorLock = 10,
    DeviceIdIoBoard = 11,
    DeviceIdChannel5 = 12,
    DeviceIdChannel6 = 13,
    DeviceIdChannel7 = 14,
    DeviceIdChannel8 = 15,
}

impl TryFrom<i32> for DeviceId {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(DeviceId::DeviceIdInvalid),
            1 => Ok(DeviceId::DeviceIdChannel1),
            2 => Ok(DeviceId::DeviceIdChannel2),
            3 => Ok(DeviceId::DeviceIdChannel3),
            4 => Ok(DeviceId::DeviceIdChannel4),
            5 => Ok(DeviceId::DeviceIdGripper),
            6 => Ok(DeviceId::DeviceIdBarcodeReader),
            7 => Ok(DeviceId::DeviceIdShiftAndScan),
            8 => Ok(DeviceId::DeviceIdBdPuncture),
            9 => Ok(DeviceId::DeviceIdSiemensCarousel),
            10 => Ok(DeviceId::DeviceIdDoorLock),
            11 => Ok(DeviceId::DeviceIdIoBoard),
            12 => Ok(DeviceId::DeviceIdChannel5),
            13 => Ok(DeviceId::DeviceIdChannel6),
            14 => Ok(DeviceId::DeviceIdChannel7),
            15 => Ok(DeviceId::DeviceIdChannel8),

            _ => Err(ConnectionError(anyhow!("Unknown DeviceId value {}", v))),
        }
    }
}

impl PigletCodec for DeviceId {
    const TYPE_ID: u8 = 32;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self as i32);
    }

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

impl PigletCodec for MVec<Vec<DeviceId>> {
    const TYPE_ID: u8 = 35;
    fn serialize(&self, bytes: &mut BytesMut) {
        bytes.put_u8(Self::TYPE_ID);
        bytes.put_u8(0);
        bytes.put_u16_le(4 * self.0.len() as u16);
        for v in &self.0 {
            bytes.put_i32_le(*v as i32);
        }
    }

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

#[derive(Clone, Debug)]
pub struct ShiftAndScanRackReply {
    x_positions: Vec<i32>,
    barcodes: String,
}

#[derive(Clone, Debug)]
pub struct ShiftAndScanRowReply {
    barcodes: String,
}

#[derive(Clone, Debug)]
pub struct GetChannelConfiguration_1Reply {
    channels: u16,
    channel_types: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetChannelConfiguration_2Reply {
    channels: u16,
    channel_types: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetShiftAndScanSensorsReply {
    sensor_1: bool,
    sensor_2: bool,
    sensor_3: bool,
}

#[derive(Clone, Debug)]
pub struct ShiftAndScanRackXSpeedReply {
    x_positions: Vec<i32>,
    barcodes: String,
}

#[derive(Clone, Debug)]
pub struct GetXVelocityLimitsReply {
    lower_limit: u32,
    upper_limit: u32,
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
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
    enumeration_values: Vec<i32>,
    enumeration_value_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}
