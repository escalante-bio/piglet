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

pub struct NimbusCoreCpu {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreCpu {
    // version 1.0

    pub fn new_1(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49152,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_2(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49408,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_3(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49409,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_4(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49410,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_5(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49411,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_6(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49412,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_7(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49413,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_8(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49414,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_9(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 49415,
            },
            robot: robot.clone(),
        }
    }

    pub async fn read_uint_32(&self, address: u32) -> Result</* value= */ u32, Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 8, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u32::deserialize(&mut stream)?;
        Ok(value)
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

    pub async fn reg_table_entries(&self) -> Result</* entries= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 11, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let entries = u32::deserialize(&mut stream)?;
        Ok(entries)
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

    pub async fn get_serial_number(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 9, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn get_module_version(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 8, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn reg_table_entry(&self, entry: u32) -> Result<RegTableEntryReply, Error> {
        let mut args = BytesMut::new();
        entry.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 12, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let module_id = u16::deserialize(&mut stream)?;
        let node_id = u16::deserialize(&mut stream)?;
        let object_id = u16::deserialize(&mut stream)?;
        let link_handle = u32::deserialize(&mut stream)?;
        Ok(RegTableEntryReply {
            module_id,
            node_id,
            object_id,
            link_handle,
        })
    }

    pub async fn get_test_address(&self) -> Result</* value= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 2, 0, 4, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u32::deserialize(&mut stream)?;
        Ok(value)
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

    pub async fn get_firmware_revision(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 6, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
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

    pub async fn get_other(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 12, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn is_in_boot(&self) -> Result</* in_boot= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 6, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let in_boot = bool::deserialize(&mut stream)?;
        Ok(in_boot)
    }

    pub async fn reset(&self, delay_ms: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        delay_ms.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 2, 3, 1, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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

    pub async fn get_operating_mode(&self) -> Result</* value= */ OperatingMode, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 2, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = OperatingMode::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn get_up_time(&self) -> Result</* value= */ SUpTime, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 2, 0, 3, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = SUpTime::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn write_uint_8(&self, address: u32, value: u8) -> Result<(), Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        value.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_firmware_version(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 3, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn read_uint_8(&self, address: u32) -> Result</* value= */ u8, Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u8::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn version(&self) -> Result<VersionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 5, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let firmware_version = String::deserialize(&mut stream)?;
        Ok(VersionReply { firmware_version })
    }

    pub async fn download_write(&self, download_data: Vec<u8>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        download_data.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn download_complete(&self, success: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        success.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 4, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_hardware_revision(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 11, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
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

    pub async fn boot_loader_version(&self) -> Result</* boot_loader_version= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 2, 0, 2, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let boot_loader_version = String::deserialize(&mut stream)?;
        Ok(boot_loader_version)
    }

    pub async fn get_calibration_information(
        &self,
    ) -> Result</* value= */ Vec<CalibrationInformation>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 7, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = MVec::<Vec<CalibrationInformation>>::deserialize(&mut stream)?.0;
        Ok(value)
    }

    pub async fn get_firmware_part_number(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 5, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn get_module_name(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 1, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn download_initiate(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 2, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn write_uint_32(&self, address: u32, value: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        value.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 10, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_compatibility_string(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 4, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn download_info(&self) -> Result<DownloadInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 1, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let buffer_size = i32::deserialize(&mut stream)?;
        let file_name_template = String::deserialize(&mut stream)?;
        Ok(DownloadInfoReply {
            buffer_size,
            file_name_template,
        })
    }

    pub async fn get_hardware_part_number(&self) -> Result</* value= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 3, 0, 10, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = String::deserialize(&mut stream)?;
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OperatingMode {
    Bootloader = 0,
    Runtime = 1,
}

impl TryFrom<i32> for OperatingMode {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(OperatingMode::Bootloader),
            1 => Ok(OperatingMode::Runtime),

            _ => Err(ConnectionError(anyhow!(
                "Unknown OperatingMode value {}",
                v
            ))),
        }
    }
}

impl PigletCodec for OperatingMode {
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

impl PigletCodec for MVec<Vec<OperatingMode>> {
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
pub struct SUpTime {
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

impl PigletCodec for SUpTime {
    const TYPE_ID: u8 = 30;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.days.serialize(&mut buffer);
        s.hours.serialize(&mut buffer);
        s.minutes.serialize(&mut buffer);
        s.seconds.serialize(&mut buffer);
        s.milliseconds.serialize(&mut buffer);

        stream.put_u16_le(buffer.len() as u16);
        stream.put(buffer);
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
        Ok(Self {
            days: u8::deserialize(&mut bytes)?,
            hours: u8::deserialize(&mut bytes)?,
            minutes: u8::deserialize(&mut bytes)?,
            seconds: u8::deserialize(&mut bytes)?,
            milliseconds: u16::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MVec<Vec<SUpTime>> {
    const TYPE_ID: u8 = 31;
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in &self.0 {
            let mut buffer = BytesMut::new();

            s.days.serialize(&mut buffer);
            s.hours.serialize(&mut buffer);
            s.minutes.serialize(&mut buffer);
            s.seconds.serialize(&mut buffer);
            s.milliseconds.serialize(&mut buffer);

            outer.put_u16_le(buffer.len() as u16);
            outer.put(buffer);
        }

        stream.put_u16_le(outer.len() as u16);
        stream.put(outer);
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
        let mut length = stream.get_u16_le() as usize;
        let mut outer = stream.copy_to_bytes(length);
        let mut arr = Vec::new();
        while length > 0 {
            let mut length = outer.get_u16_le() as usize;
            let mut bytes = outer.copy_to_bytes(length);
            arr.push(SUpTime {
                days: u8::deserialize(&mut bytes)?,
                hours: u8::deserialize(&mut bytes)?,
                minutes: u8::deserialize(&mut bytes)?,
                seconds: u8::deserialize(&mut bytes)?,
                milliseconds: u16::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
}

#[derive(Clone, Debug)]
pub struct CalibrationInformation {
    pub description: String,
    pub date: NetworkResult,
}

impl PigletCodec for CalibrationInformation {
    const TYPE_ID: u8 = 30;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.description.serialize(&mut buffer);
        s.date.serialize(&mut buffer);

        stream.put_u16_le(buffer.len() as u16);
        stream.put(buffer);
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
        Ok(Self {
            description: String::deserialize(&mut bytes)?,
            date: NetworkResult::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MVec<Vec<CalibrationInformation>> {
    const TYPE_ID: u8 = 31;
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in &self.0 {
            let mut buffer = BytesMut::new();

            s.description.serialize(&mut buffer);
            s.date.serialize(&mut buffer);

            outer.put_u16_le(buffer.len() as u16);
            outer.put(buffer);
        }

        stream.put_u16_le(outer.len() as u16);
        stream.put(outer);
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
        let mut length = stream.get_u16_le() as usize;
        let mut outer = stream.copy_to_bytes(length);
        let mut arr = Vec::new();
        while length > 0 {
            let mut length = outer.get_u16_le() as usize;
            let mut bytes = outer.copy_to_bytes(length);
            arr.push(CalibrationInformation {
                description: String::deserialize(&mut bytes)?,
                date: NetworkResult::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
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

#[derive(Clone, Debug)]
pub struct RegTableEntryReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
    link_handle: u32,
}

#[derive(Clone, Debug)]
pub struct InterfaceDescriptorsReply {
    interface_ids: Vec<u8>,
    interface_descriptors: Vec<String>,
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
pub struct VersionReply {
    firmware_version: String,
}

#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    enumeration_names: Vec<String>,
    number_enumeration_values: Vec<u32>,
    enumeration_values: Vec<i16>,
    enumeration_value_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DownloadInfoReply {
    buffer_size: i32,
    file_name_template: String,
}
