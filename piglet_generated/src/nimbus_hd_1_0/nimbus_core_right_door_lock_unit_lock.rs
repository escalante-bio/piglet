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

pub struct NimbusCoreRightDoorLockUnitLock {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreRightDoorLockUnitLock {
    // version

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 129,
                object_id: 256,
            },
            robot: robot.clone(),
        }
    }
}

// module was not present on the dumping machine
