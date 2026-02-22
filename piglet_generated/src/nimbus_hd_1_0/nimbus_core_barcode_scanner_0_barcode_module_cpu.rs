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
pub struct NimbusCoreBarcodeScanner0BarcodeModuleCpu {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreBarcodeScanner0BarcodeModuleCpu {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 96,
                object_id: 49408,
            },
            robot: robot.clone(),
        }
    }

    pub async fn download_info(&self) -> Result<DownloadInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.DownloadInfo()".to_string(),
        )?;

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

    pub async fn download_initiate(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || {
                "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.DownloadInitiate()"
                    .to_string()
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn download_write(&self, download_data: impl AsRef<[u8]>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        download_data.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || {
                let parameters = vec![format!("  download_data: {:?}", download_data.as_ref())];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.DownloadWrite(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn download_complete(&self, success: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        success.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || {
                let parameters = vec![format!("  success: {:?}", success)];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.DownloadComplete(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn version(&self) -> Result<VersionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 5, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.Version()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let firmware_version = String::deserialize(&mut stream)?;
        Ok(VersionReply { firmware_version })
    }

    pub async fn is_in_boot(&self) -> Result</* in_boot= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 6, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.IsInBoot()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let in_boot = bool::deserialize(&mut stream)?;
        Ok(in_boot)
    }

    pub async fn read_uint_8(&self, address: u32) -> Result</* value= */ u8, Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![format!("  address: {:?}", address)];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.ReadUINT8(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u8::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn read_uint_32(&self, address: u32) -> Result</* value= */ u32, Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![format!("  address: {:?}", address)];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.ReadUINT32(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u32::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn write_uint_8(&self, address: u32, value: u8) -> Result<(), Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        value.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 9, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  address: {:?}", address),
                    format!("  value: {:?}", value),
                ];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.WriteUINT8(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn write_uint_32(&self, address: u32, value: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        address.serialize(&mut args);
        value.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 10, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  address: {:?}", address),
                    format!("  value: {:?}", value),
                ];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.WriteUINT32(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn reg_table_entries(&self) -> Result</* entries= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 11, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.RegTableEntries()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let entries = u32::deserialize(&mut stream)?;
        Ok(entries)
    }

    pub async fn reg_table_entry(&self, entry: u32) -> Result<RegTableEntryReply, Error> {
        let mut args = BytesMut::new();
        entry.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 12, args.freeze()).await,
            || {
                let parameters = vec![format!("  entry: {:?}", entry)];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.RegTableEntry(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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

    pub async fn reset(&self, delay_ms: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        delay_ms.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 3, 1, args.freeze()).await,
            || {
                let parameters = vec![format!("  delay_ms: {:?}", delay_ms)];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.Reset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn boot_loader_version(&self) -> Result</* boot_loader_version= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 0, 2, args.freeze()).await,
            || {
                "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.BootLoaderVersion()"
                    .to_string()
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let boot_loader_version = String::deserialize(&mut stream)?;
        Ok(boot_loader_version)
    }

    pub async fn get_up_time(&self) -> Result</* value= */ SUpTime, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 0, 3, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.GetUpTime()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = SUpTime::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn get_test_address(&self) -> Result</* value= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 0, 4, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.GetTestAddress()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let value = u32::deserialize(&mut stream)?;
        Ok(value)
    }

    pub async fn get_compression_algorithm(
        &self,
    ) -> Result</* algorithm= */ CompressionAlgorithm, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 0, 5, args.freeze()).await,
            || {
                "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.GetCompressionAlgorithm()"
                    .to_string()
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let algorithm = CompressionAlgorithm::deserialize(&mut stream)?;
        Ok(algorithm)
    }

    pub async fn download_write_compressed_data(
        &self,

        download_data: impl AsRef<[u8]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        download_data.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![format!("  download_data: {:?}", download_data.as_ref())];
                format!(
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.DownloadWriteCompressedData(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_download_timeouts(&self) -> Result<GetDownloadTimeoutsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 2, 0, 7, args.freeze()).await,
            || {
                "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.GetDownloadTimeouts()"
                    .to_string()
            },
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let download_write_timeout = u32::deserialize(&mut stream)?;
        let download_complete_timeout = u32::deserialize(&mut stream)?;
        Ok(GetDownloadTimeoutsReply {
            download_write_timeout,
            download_complete_timeout,
        })
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.SubObjectInfo(\n{}\n)",
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
            || {
                "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.InterfaceDescriptors()"
                    .to_string()
            },
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
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreBarcodeScanner0BarcodeModuleCpu.StructInfo(\n{}\n)",
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
pub enum CompressionAlgorithm {
    Nocompression = 0,
    Hexcompression = 1,
    Fastlz = 2,
}

impl TryFrom<i32> for CompressionAlgorithm {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(CompressionAlgorithm::Nocompression),
            1 => Ok(CompressionAlgorithm::Hexcompression),
            2 => Ok(CompressionAlgorithm::Fastlz),

            _ => Err(ConnectionError(anyhow!(
                "Unknown CompressionAlgorithm value {}",
                v
            ))),
        }
    }
}

impl PigletCodec for CompressionAlgorithm {
    const TYPE_ID: u8 = 32;
}

impl PigletDeserialize for CompressionAlgorithm {
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

impl PigletSerialize for CompressionAlgorithm {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self as i32);
    }
}

impl PigletCodec for MSlice<'_, CompressionAlgorithm> {
    const TYPE_ID: u8 = 35;
}

impl PigletCodec for MVec<CompressionAlgorithm> {
    const TYPE_ID: u8 = 35;
}

impl PigletDeserialize for MVec<CompressionAlgorithm> {
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

impl PigletSerialize for MSlice<'_, CompressionAlgorithm> {
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
pub struct SUpTime {
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

impl PigletCodec for SUpTime {
    const TYPE_ID: u8 = 30;
}

impl PigletSerialize for SUpTime {
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
}

impl PigletDeserialize for SUpTime {
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

impl PigletCodec for MSlice<'_, SUpTime> {
    const TYPE_ID: u8 = 31;
}

impl PigletCodec for MVec<SUpTime> {
    const TYPE_ID: u8 = 31;
}

impl PigletSerialize for MSlice<'_, SUpTime> {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in self.0.as_ref() {
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
}

impl PigletSerialize for MVec<SUpTime> {
    fn serialize(&self, stream: &mut BytesMut) {
        MSlice(&self.0).serialize(stream)
    }
}

impl PigletDeserialize for MVec<SUpTime> {
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

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct DownloadInfoReply {
    pub buffer_size: i32,
    pub file_name_template: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct VersionReply {
    pub firmware_version: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct RegTableEntryReply {
    pub module_id: u16,
    pub node_id: u16,
    pub object_id: u16,
    pub link_handle: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetDownloadTimeoutsReply {
    pub download_write_timeout: u32,
    pub download_complete_timeout: u32,
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
