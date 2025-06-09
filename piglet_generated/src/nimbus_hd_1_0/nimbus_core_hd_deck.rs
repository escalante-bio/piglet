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

pub struct NimbusCoreHdDeck {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreHdDeck {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 266,
            },
            robot: robot.clone(),
        }
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

    pub async fn get_tracks(&self) -> Result</* track= */ u8, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 2, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let track = u8::deserialize(&mut stream)?;
        Ok(track)
    }

    pub async fn get_gantry_status(&self) -> Result</* status= */ GantryState, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 4, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let status = GantryState::deserialize(&mut stream)?;
        Ok(status)
    }

    pub async fn get_track_led_states(&self) -> Result</* leds= */ Vec<LedState>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 8, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let leds = MVec::<Vec<LedState>>::deserialize(&mut stream)?.0;
        Ok(leds)
    }

    pub async fn have_deck_sensors_changed(&self) -> Result</* changed= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 12, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let changed = bool::deserialize(&mut stream)?;
        Ok(changed)
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

    pub async fn configure_tracks(
        &self,
        configuring: bool,
    ) -> Result</* sensors= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        configuring.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 5, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensors = Vec::<bool>::deserialize(&mut stream)?;
        Ok(sensors)
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

    pub async fn load_tracks(
        &self,
        load: bool,
        track: u8,
        width: u8,
    ) -> Result</* sensors= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        load.serialize(&mut args);
        track.serialize(&mut args);
        width.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 6, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensors = Vec::<bool>::deserialize(&mut stream)?;
        Ok(sensors)
    }

    pub async fn load_tracks_2(
        &self,
        load: bool,
        tracks: Vec<u8>,
        widths: Vec<u8>,
    ) -> Result</* sensors= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        load.serialize(&mut args);
        tracks.serialize(&mut args);
        widths.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensors = Vec::<bool>::deserialize(&mut stream)?;
        Ok(sensors)
    }

    pub async fn is_deck_monitored(&self) -> Result</* monitoring= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 10, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let monitoring = bool::deserialize(&mut stream)?;
        Ok(monitoring)
    }

    pub async fn monitor_deck(&self, monitor: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        monitor.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
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

    pub async fn configure_track_leds(&self, leds: Vec<LedConfiguration>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        MVec(leds).serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
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

    pub async fn cancel_track(&self) -> Result</* sensors= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensors = Vec::<bool>::deserialize(&mut stream)?;
        Ok(sensors)
    }

    pub async fn is_deck_monitoring_available(&self) -> Result</* monitoring= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 1, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let monitoring = bool::deserialize(&mut stream)?;
        Ok(monitoring)
    }

    pub async fn get_track_sensor_states(&self) -> Result</* sensors= */ Vec<bool>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 3, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensors = Vec::<bool>::deserialize(&mut stream)?;
        Ok(sensors)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GantryState {
    Normal = 0,
    Locked = 1,
}

impl TryFrom<i32> for GantryState {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(GantryState::Normal),
            1 => Ok(GantryState::Locked),

            _ => Err(ConnectionError(anyhow!("Unknown GantryState value {}", v))),
        }
    }
}

impl PigletCodec for GantryState {
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

impl PigletCodec for MVec<Vec<GantryState>> {
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

#[derive(Clone, Copy, Debug)]
pub enum LedState {
    Empty = 0,
    Loaded = 1,
    Loading = 2,
    Unloading = 3,
}

impl TryFrom<i32> for LedState {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(LedState::Empty),
            1 => Ok(LedState::Loaded),
            2 => Ok(LedState::Loading),
            3 => Ok(LedState::Unloading),

            _ => Err(ConnectionError(anyhow!("Unknown LedState value {}", v))),
        }
    }
}

impl PigletCodec for LedState {
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

impl PigletCodec for MVec<Vec<LedState>> {
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
pub struct LedConfiguration {
    pub position: u8,
    pub state: LedState,
}

impl PigletCodec for LedConfiguration {
    const TYPE_ID: u8 = 30;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.position.serialize(&mut buffer);
        s.state.serialize(&mut buffer);

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
            position: u8::deserialize(&mut bytes)?,
            state: LedState::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MVec<Vec<LedConfiguration>> {
    const TYPE_ID: u8 = 31;
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in &self.0 {
            let mut buffer = BytesMut::new();

            s.position.serialize(&mut buffer);
            s.state.serialize(&mut buffer);

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
            arr.push(LedConfiguration {
                position: u8::deserialize(&mut bytes)?,
                state: LedState::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
}

#[derive(Clone, Debug)]
pub struct EventTrackSensors {
    pub sensors: Vec<bool>,
}

impl PigletCodec for EventTrackSensors {
    const TYPE_ID: u8 = 30;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.sensors.serialize(&mut buffer);

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
            sensors: Vec::<bool>::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MVec<Vec<EventTrackSensors>> {
    const TYPE_ID: u8 = 31;
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in &self.0 {
            let mut buffer = BytesMut::new();

            s.sensors.serialize(&mut buffer);

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
            arr.push(EventTrackSensors {
                sensors: Vec::<bool>::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
}

#[derive(Clone, Debug)]
pub struct EventGantryState {
    pub state: GantryState,
}

impl PigletCodec for EventGantryState {
    const TYPE_ID: u8 = 30;

    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        let mut buffer = BytesMut::new();
        let s = self;

        s.state.serialize(&mut buffer);

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
            state: GantryState::deserialize(&mut bytes)?,
        })
    }
}

impl PigletCodec for MVec<Vec<EventGantryState>> {
    const TYPE_ID: u8 = 31;
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);

        let mut outer = BytesMut::new();
        for s in &self.0 {
            let mut buffer = BytesMut::new();

            s.state.serialize(&mut buffer);

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
            arr.push(EventGantryState {
                state: GantryState::deserialize(&mut bytes)?,
            });
        }
        Ok(MVec(arr))
    }
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
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
pub struct MethodInfoReply {
    interfaceid: u8,
    action: u8,
    actionid: u16,
    name: String,
    parametertypes: String,
    parameternames: String,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
}
