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
pub struct NimbusCoreIoBoardCpu {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreIoBoardCpu {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 96,
                node_id: 1,
                object_id: 49408,
            },
            robot: robot.clone(),
        }
    }

    pub async fn download_info(&self) -> Result<DownloadInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreIoBoardCpu.DownloadInfo()".to_string(),
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
            || "in call to NimbusCoreIoBoardCpu.DownloadInitiate()".to_string(),
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
                    "in call to NimbusCoreIoBoardCpu.DownloadWrite(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.DownloadComplete(\n{}\n)",
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
            || "in call to NimbusCoreIoBoardCpu.Version()".to_string(),
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
            || "in call to NimbusCoreIoBoardCpu.IsInBoot()".to_string(),
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
                    "in call to NimbusCoreIoBoardCpu.ReadUINT8(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.ReadUINT32(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.WriteUINT8(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.WriteUINT32(\n{}\n)",
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
            || "in call to NimbusCoreIoBoardCpu.RegTableEntries()".to_string(),
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
                    "in call to NimbusCoreIoBoardCpu.RegTableEntry(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.Reset(\n{}\n)",
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
            || "in call to NimbusCoreIoBoardCpu.BootLoaderVersion()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let boot_loader_version = String::deserialize(&mut stream)?;
        Ok(boot_loader_version)
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreIoBoardCpu.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreIoBoardCpu.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreIoBoardCpu.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreIoBoardCpu.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreIoBoardCpu.StructInfo(\n{}\n)",
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
pub struct DownloadInfoReply {
    buffer_size: i32,
    file_name_template: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct VersionReply {
    firmware_version: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct RegTableEntryReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
    link_handle: u32,
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
