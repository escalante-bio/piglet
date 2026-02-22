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
pub struct NimbusCoreLeftDoorLockUnitCpu {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreLeftDoorLockUnitCpu {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 128,
                object_id: 49408,
            },
            robot: robot.clone(),
        }
    }
}

// module was not present on the dumping machine
