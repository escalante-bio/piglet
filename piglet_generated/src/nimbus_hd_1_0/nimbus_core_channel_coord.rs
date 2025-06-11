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

pub struct NimbusCoreChannelCoord {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreChannelCoord {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 262,
            },
            robot: robot.clone(),
        }
    }

    pub async fn pickup_tips(
        &self,

        tips_used: Vec<u16>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: i32,
        tip_type: Vec<u16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_type.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 1, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_tips(
        &self,

        tips_used: Vec<u16>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 2, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate(
        &self,

        aspirate_type: Vec<i16>,
        tips_used: Vec<u16>,
        liquid_seek_height: Vec<i32>,
        liquid_surface_height: Vec<i32>,
        submerge_depth: Vec<i32>,
        follow_depth: Vec<i32>,
        z_min_position: Vec<i32>,
        clot_check_height: Vec<i32>,
        z_final: i32,
        liquid_exit_speed: Vec<u32>,
        blowout_volume: Vec<u32>,
        prewet_volume: Vec<u32>,
        aspirate_volume: Vec<u32>,
        transport_air_volume: Vec<u32>,
        aspirate_speed: Vec<u32>,
        settling_time: Vec<u32>,
        mix_volume: Vec<u32>,
        mix_cycles: Vec<u32>,
        mix_position: Vec<i32>,
        mix_follow_distance: Vec<i32>,
        mix_speed: Vec<u32>,
        aspirate_offset: Vec<i32>,
        tube_section_height: Vec<i32>,
        tube_section_ratio: Vec<i32>,
        lld_mode: Vec<i16>,
        capacitive_lld_sensitivity: Vec<i16>,
        pressure_lld_sensitivity: Vec<i16>,
        lld_height_difference: Vec<i32>,
        tadm_enabled: bool,
        limit_curve_index: Vec<u32>,
        recording_mode: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        aspirate_type.serialize(&mut args);
        tips_used.serialize(&mut args);
        liquid_seek_height.serialize(&mut args);
        liquid_surface_height.serialize(&mut args);
        submerge_depth.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        clot_check_height.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        blowout_volume.serialize(&mut args);
        prewet_volume.serialize(&mut args);
        aspirate_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        aspirate_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        mix_volume.serialize(&mut args);
        mix_cycles.serialize(&mut args);
        mix_position.serialize(&mut args);
        mix_follow_distance.serialize(&mut args);
        mix_speed.serialize(&mut args);
        aspirate_offset.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        lld_mode.serialize(&mut args);
        capacitive_lld_sensitivity.serialize(&mut args);
        pressure_lld_sensitivity.serialize(&mut args);
        lld_height_difference.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense(
        &self,

        dispense_type: Vec<i16>,
        tips_used: Vec<u16>,
        liquid_seek_height: Vec<i32>,
        dispense_height: Vec<i32>,
        submerge_depth: Vec<i32>,
        follow_depth: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        liquid_exit_speed: Vec<u32>,
        transport_air_volume: Vec<u32>,
        dispense_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        blowout_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        settling_time: Vec<u32>,
        mix_volume: Vec<u32>,
        mix_cycles: Vec<u32>,
        mix_position: Vec<i32>,
        mix_follow_distance: Vec<i32>,
        mix_speed: Vec<u32>,
        touchoff_distance: i32,
        dispense_offset: Vec<i32>,
        tube_section_height: Vec<i32>,
        tube_section_ratio: Vec<i32>,
        lld_mode: Vec<i16>,
        capacitive_lld_sensitivity: Vec<i16>,
        tadm_enabled: bool,
        limit_curve_index: Vec<u32>,
        recording_mode: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        dispense_type.serialize(&mut args);
        tips_used.serialize(&mut args);
        liquid_seek_height.serialize(&mut args);
        dispense_height.serialize(&mut args);
        submerge_depth.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        blowout_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        mix_volume.serialize(&mut args);
        mix_cycles.serialize(&mut args);
        mix_position.serialize(&mut args);
        mix_follow_distance.serialize(&mut args);
        mix_speed.serialize(&mut args);
        touchoff_distance.serialize(&mut args);
        dispense_offset.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        lld_mode.serialize(&mut args);
        capacitive_lld_sensitivity.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 4, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate_and_dispense(
        &self,

        aspirate_type: Vec<i16>,
        dispense_type: Vec<i16>,
        tips_used: Vec<u16>,
        aspirate_dispense_pattern: Vec<i16>,
        liquid_seek_height: Vec<i32>,
        liquid_surface_height: Vec<i32>,
        submerge_depth: Vec<i32>,
        follow_depth: Vec<i32>,
        z_min_position: Vec<i32>,
        clot_check_height: Vec<i32>,
        z_final: i32,
        liquid_exit_speed: Vec<u32>,
        blowout_volume: Vec<u32>,
        prewet_volume: Vec<u32>,
        aspirate_volume: Vec<u32>,
        transport_air_volume: Vec<u32>,
        dispense_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        aspirate_speed: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        settling_time: Vec<u32>,
        mix_volume: Vec<u32>,
        mix_cycles: Vec<u32>,
        mix_position: Vec<i32>,
        mix_follow_distance: Vec<i32>,
        mix_speed: Vec<u32>,
        offset: Vec<i32>,
        tube_section_height: Vec<i32>,
        tube_section_ratio: Vec<i32>,
        lld_mode: Vec<i16>,
        capacitive_lld_sensitivity: Vec<i16>,
        pressure_lld_sensitivity: Vec<i16>,
        lld_height_difference: Vec<i32>,
        tadm_enabled: bool,
        limit_curve_index: Vec<u32>,
        recording_mode: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        aspirate_type.serialize(&mut args);
        dispense_type.serialize(&mut args);
        tips_used.serialize(&mut args);
        aspirate_dispense_pattern.serialize(&mut args);
        liquid_seek_height.serialize(&mut args);
        liquid_surface_height.serialize(&mut args);
        submerge_depth.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        clot_check_height.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        blowout_volume.serialize(&mut args);
        prewet_volume.serialize(&mut args);
        aspirate_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        aspirate_speed.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        mix_volume.serialize(&mut args);
        mix_cycles.serialize(&mut args);
        mix_position.serialize(&mut args);
        mix_follow_distance.serialize(&mut args);
        mix_speed.serialize(&mut args);
        offset.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        lld_mode.serialize(&mut args);
        capacitive_lld_sensitivity.serialize(&mut args);
        pressure_lld_sensitivity.serialize(&mut args);
        lld_height_difference.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 5, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pickup_gripper_tool(
        &self,

        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        tip_type: u16,
        first_channel_number: u16,
        second_channel_number: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_type.serialize(&mut args);
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 6, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_gripper_tool(
        &self,

        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        first_channel_number: u16,
        second_channel_number: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_pre_initialize(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 8, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_initialize(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_relative(
        &self,

        tips_used: Vec<u16>,
        y_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 10, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute(
        &self,

        tips_used: Vec<u16>,
        y_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_relative_speed(
        &self,

        tips_used: Vec<u16>,
        y_distance: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_distance.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 12, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute_speed(
        &self,

        tips_used: Vec<u16>,
        y_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_seek_lld(
        &self,

        tips_used: Vec<u16>,
        y_position: Vec<i32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 14, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute_gripper(
        &self,

        tips_used: Vec<u16>,
        y_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 15, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_initialize(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 16, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_safe(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 17, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_relative(
        &self,

        tips_used: Vec<u16>,
        z_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 18, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_absolute(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 19, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_relative_speed(
        &self,

        tips_used: Vec<u16>,
        z_distance: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_distance.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 20, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_absolute_speed(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 21, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_traverse(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 22, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_lld(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
        z_min_position: Vec<i32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        z_min_position.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 23, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_absolute_gripper(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 24, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_initialize(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 25, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_empty(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 26, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_move_relative(
        &self,

        tips_used: Vec<u16>,
        volume: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        volume.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 27, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn squeeze_initialize(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 28, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn squeeze_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 29, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn lld_on(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 30, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn lld_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 31, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_y_positions(&self) -> Result</* y_positions= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 32, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_positions = Vec::<i32>::deserialize(&mut stream)?;
        Ok(y_positions)
    }

    pub async fn get_y_home_offsets(&self) -> Result</* y_home_offsets= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 33, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_home_offsets = Vec::<i32>::deserialize(&mut stream)?;
        Ok(y_home_offsets)
    }

    pub async fn set_y_home_offsets(
        &self,

        tips_used: Vec<u16>,
        y_home_offsets: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_home_offsets.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 34, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_positions(&self) -> Result</* z_positions= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 35, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_positions = Vec::<i32>::deserialize(&mut stream)?;
        Ok(z_positions)
    }

    pub async fn get_z_home_offsets(&self) -> Result</* z_home_offsets= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 36, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_home_offsets = Vec::<i32>::deserialize(&mut stream)?;
        Ok(z_home_offsets)
    }

    pub async fn set_z_home_offsets(
        &self,

        tips_used: Vec<u16>,
        z_home_offsets: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_home_offsets.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 37, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_init_status(&self) -> Result</* initialization_status= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 38, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialization_status = Vec::<i16>::deserialize(&mut stream)?;
        Ok(initialization_status)
    }

    pub async fn get_tip_status(&self) -> Result</* tip_present= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 39, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_present = Vec::<i16>::deserialize(&mut stream)?;
        Ok(tip_present)
    }

    pub async fn get_lld_status(&self) -> Result</* lld_status= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 40, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let lld_status = Vec::<i16>::deserialize(&mut stream)?;
        Ok(lld_status)
    }

    pub async fn calibrate_dispense_check(
        &self,

        tips_used: Vec<u16>,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        volume: u32,
        collet_check: i16,
    ) -> Result</* steps= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 41, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let steps = Vec::<i32>::deserialize(&mut stream)?;
        Ok(steps)
    }

    pub async fn calibrate_position_reset(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 42, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_squeeze(
        &self,

        tips_used: Vec<u16>,
        z_start_position: i32,
        z_stop_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 43, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_tip_height(
        &self,

        tips_used: Vec<u16>,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        tip_volume: u32,
        collet_check: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 44, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_touchoff(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 45, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn erase_limit_curves(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 46, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn create_limit_curve(
        &self,

        channel: u16,
        name: String,
        lower_limit_x: Vec<u16>,
        lower_limit_y: Vec<i16>,
        upper_limit_x: Vec<u16>,
        upper_limit_y: Vec<i16>,
    ) -> Result<CreateLimitCurveReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        name.serialize(&mut args);
        lower_limit_x.serialize(&mut args);
        lower_limit_y.serialize(&mut args);
        upper_limit_x.serialize(&mut args);
        upper_limit_y.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 47, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let index = u32::deserialize(&mut stream)?;
        Ok(CreateLimitCurveReply { index })
    }

    pub async fn reset_tadm_fifo(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 48, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn retrieve_tadm_data(
        &self,

        tips_used: Vec<u16>,
    ) -> Result<RetrieveTadmDataReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 49, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let offsets = Vec::<u16>::deserialize(&mut stream)?;
        let tadm_data = Vec::<i16>::deserialize(&mut stream)?;
        Ok(RetrieveTadmDataReply { offsets, tadm_data })
    }

    pub async fn get_limit_curve_names(
        &self,

        channel: u16,
    ) -> Result<GetLimitCurveNamesReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 50, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let names = String::deserialize(&mut stream)?;
        Ok(GetLimitCurveNamesReply { names })
    }

    pub async fn get_limit_curve_info(
        &self,

        channel: u16,
        name: String,
    ) -> Result<GetLimitCurveInfoReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        name.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 51, args.freeze())
            .await?;
        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let index = u32::deserialize(&mut stream)?;
        let lower_limits = u16::deserialize(&mut stream)?;
        let upper_limits = u16::deserialize(&mut stream)?;
        Ok(GetLimitCurveInfoReply {
            index,
            lower_limits,
            upper_limits,
        })
    }

    pub async fn set_tadm_enable(
        &self,

        tips_used: Vec<u16>,
        enable: Vec<i16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        enable.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 52, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_tadm_enable(&self) -> Result<GetTADMEnableReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 53, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetTADMEnableReply { enable })
    }

    pub async fn enable_mad(
        &self,

        tips_used: Vec<u16>,
        pressure_threshold: Vec<u16>,
        minimum_pressure_difference: Vec<u16>,
        maximum_pressure_difference: Vec<u16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        pressure_threshold.serialize(&mut args);
        minimum_pressure_difference.serialize(&mut args);
        maximum_pressure_difference.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 54, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn disable_mad(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 55, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn enable_adc(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 56, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn disable_adc(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 57, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_z_default_speed(
        &self,

        tips_used: Vec<u16>,
        speed: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 58, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_default_speed(&self) -> Result<GetZDefaultSpeedReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 59, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let speed = Vec::<i32>::deserialize(&mut stream)?;
        Ok(GetZDefaultSpeedReply { speed })
    }

    pub async fn z_seek_obstacle(
        &self,

        tips_used: Vec<u16>,
        obstacle_seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
    ) -> Result</* obstacle_detected= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        obstacle_seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 60, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let obstacle_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(obstacle_detected)
    }

    pub async fn get_pressure(&self) -> Result</* pressures= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 61, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let pressures = Vec::<i16>::deserialize(&mut stream)?;
        Ok(pressures)
    }

    pub async fn get_potentiometer_settings(&self) -> Result<GetPotentiometerSettingsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 62, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let gain = Vec::<i16>::deserialize(&mut stream)?;
        let offset = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetPotentiometerSettingsReply { gain, offset })
    }

    pub async fn squeeze_on(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 63, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_measurement_mode(&self, channel: u16, mode: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 64, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_sensor_gain(&self, channel: u16, gain: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        gain.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 65, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_sensor_offset(&self, channel: u16, offset: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        offset.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 66, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_sensor_shift(&self, channel: u16, shift: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        shift.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 67, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn store_pressure_sensor_gain(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 68, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn store_pressure_sensor_offset(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 69, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_tadm_offset(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 70, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn prepare_on_the_fly_dispense(
        &self,

        tips_used: Vec<u16>,
        volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        transport_air_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        tadm_enabled: Vec<i16>,
        limit_curve_index: Vec<u32>,
        dispense_pattern: Vec<u16>,
        recording_mode: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        dispense_pattern.serialize(&mut args);
        recording_mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 71, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn start_on_the_fly_dispense(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 72, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn stop_on_the_fly_dispense(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 73, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn read_on_the_fly_dispense_errors(
        &self,

        tips_used: Vec<u16>,
    ) -> Result<ReadOnTheFlyDispenseErrorsReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 74, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let error = bool::deserialize(&mut stream)?;
        let results = Vec::<u16>::deserialize(&mut stream)?;
        Ok(ReadOnTheFlyDispenseErrorsReply { error, results })
    }

    pub async fn send_synch_signal(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 75, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_aspirate(
        &self,

        tips_used: Vec<u16>,
        aspirate_volume: Vec<u32>,
        aspirate_speed: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        aspirate_volume.serialize(&mut args);
        aspirate_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 76, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_dispense(
        &self,

        tips_used: Vec<u16>,
        dispense_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        dispense_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 77, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_current_dispenser_volume(&self) -> Result</* volume= */ Vec<u32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 78, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let volume = Vec::<u32>::deserialize(&mut stream)?;
        Ok(volume)
    }

    pub async fn get_liquid_height(&self) -> Result</* liquid_height= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 79, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let liquid_height = Vec::<i32>::deserialize(&mut stream)?;
        Ok(liquid_height)
    }

    pub async fn get_channel_configuration(
        &self,

        channel: u16,
        indexes: Vec<i16>,
    ) -> Result<GetChannelConfigurationReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        indexes.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 80, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enabled = Vec::<bool>::deserialize(&mut stream)?;
        Ok(GetChannelConfigurationReply { enabled })
    }

    pub async fn set_channel_configuration(
        &self,

        channel: u16,
        indexes: Vec<i16>,
        enables: Vec<bool>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        indexes.serialize(&mut args);
        enables.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 81, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn leak_check(
        &self,

        tips_used: Vec<u16>,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
        tip_type: Vec<u16>,
        time: Vec<u32>,
        test_type: Vec<bool>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_type.serialize(&mut args);
        time.serialize(&mut args);
        test_type.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 82, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_lld(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 83, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_obstacle_position(
        &self,

        tips_used: Vec<u16>,
        obstacle_seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
    ) -> Result<ZSeekObstaclePositionReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        obstacle_seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 84, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let z_position = Vec::<i32>::deserialize(&mut stream)?;
        let obstacle_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(ZSeekObstaclePositionReply {
            z_position,
            obstacle_detected,
        })
    }

    pub async fn z_seek_lld_position(
        &self,

        tips_used: Vec<u16>,
        seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
        capacitive_lld_sensitivity: Vec<i16>,
    ) -> Result<ZSeekLldPositionReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        capacitive_lld_sensitivity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 85, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let z_position = Vec::<i32>::deserialize(&mut stream)?;
        let lld_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(ZSeekLldPositionReply {
            z_position,
            lld_detected,
        })
    }

    pub async fn calibration_squeeze_check_torque(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 86, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_squeeze_position(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 87, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_z_liquid_seek_speed(
        &self,

        tips_used: Vec<u16>,
        seek_speeds: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        seek_speeds.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 88, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_liquid_seek_speed(&self) -> Result</* seek_speeds= */ Vec<u32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 89, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let seek_speeds = Vec::<u32>::deserialize(&mut stream)?;
        Ok(seek_speeds)
    }

    pub async fn z_move_absolute_2_speed(
        &self,

        tips_used: Vec<u16>,
        z_position: Vec<i32>,
        z_speed_switch_position: Vec<i32>,
        velocity: Vec<u32>,
        lower_section_velocity: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        z_speed_switch_position.serialize(&mut args);
        velocity.serialize(&mut args);
        lower_section_velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 90, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_servo_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 91, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_servo_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 92, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn save_counters(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 93, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_home_sensors(&self) -> Result<GetHomeSensorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 94, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let y_home = Vec::<bool>::deserialize(&mut stream)?;
        let z_home = Vec::<bool>::deserialize(&mut stream)?;
        let squeeze_home = Vec::<bool>::deserialize(&mut stream)?;
        let dispense_home = Vec::<bool>::deserialize(&mut stream)?;
        Ok(GetHomeSensorsReply {
            y_home,
            z_home,
            squeeze_home,
            dispense_home,
        })
    }

    pub async fn get_calibration_values(&self) -> Result<GetCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 95, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let dispense_drive = Vec::<i16>::deserialize(&mut stream)?;
        let squeeze_drive = Vec::<i16>::deserialize(&mut stream)?;
        let pressure_lld = Vec::<i16>::deserialize(&mut stream)?;
        let clot_detection = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetCalibrationValuesReply {
            dispense_drive,
            squeeze_drive,
            pressure_lld,
            clot_detection,
        })
    }

    pub async fn get_extended_calibration_values(
        &self,
    ) -> Result<GetExtendedCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 96, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_type_adjustment = Vec::<i16>::deserialize(&mut stream)?;
        let pressure_conversion = Vec::<i16>::deserialize(&mut stream)?;
        let aspirate_monitoring = Vec::<i16>::deserialize(&mut stream)?;
        let z_bottom_search = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetExtendedCalibrationValuesReply {
            tip_type_adjustment,
            pressure_conversion,
            aspirate_monitoring,
            z_bottom_search,
        })
    }

    pub async fn get_counters(&self) -> Result<GetCountersReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 97, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_pickup_counter = Vec::<u32>::deserialize(&mut stream)?;
        let tip_eject_counter = Vec::<u32>::deserialize(&mut stream)?;
        let aspirate_counter = Vec::<u32>::deserialize(&mut stream)?;
        let dispense_counter = Vec::<u32>::deserialize(&mut stream)?;
        Ok(GetCountersReply {
            tip_pickup_counter,
            tip_eject_counter,
            aspirate_counter,
            dispense_counter,
        })
    }

    pub async fn get_technical_info(
        &self,

        channel: u16,
    ) -> Result</* channel_info= */ String, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 98, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channel_info = String::deserialize(&mut stream)?;
        Ok(channel_info)
    }

    pub async fn set_counters(
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
            .act(&self.address, 1, 3, 99, args.freeze())
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
            .act(&self.address, 1, 3, 100, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let detects_per_minute = u16::deserialize(&mut stream)?;
        Ok(detects_per_minute)
    }

    pub async fn y_move_absolute_stagger(
        &self,

        tips_used: Vec<u16>,
        y_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
        delay: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        delay.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 101, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute_gripper_stagger(
        &self,

        tips_used: Vec<u16>,
        gripper_tips_used: Vec<u16>,
        y_position: Vec<i32>,
        acceleration: Vec<u32>,
        velocity: Vec<u32>,
        delay: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        gripper_tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        delay.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 102, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_positions(&self) -> Result<GetPositionsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 103, args.freeze())
            .await?;
        if count != 8 {
            return Err(ConnectionError(anyhow!("Expected 8 values, not {}", count)));
        }
        let y_position = Vec::<i32>::deserialize(&mut stream)?;
        let y_encoder_position = Vec::<i32>::deserialize(&mut stream)?;
        let z_position = Vec::<i32>::deserialize(&mut stream)?;
        let z_encoder_position = Vec::<i32>::deserialize(&mut stream)?;
        let d_position = Vec::<i32>::deserialize(&mut stream)?;
        let d_encoder_position = Vec::<i32>::deserialize(&mut stream)?;
        let s_position = Vec::<i32>::deserialize(&mut stream)?;
        let s_encoder_position = Vec::<i32>::deserialize(&mut stream)?;
        Ok(GetPositionsReply {
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

    pub async fn get_tip_types(&self) -> Result</* tip_present= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 104, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_present = Vec::<i16>::deserialize(&mut stream)?;
        Ok(tip_present)
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

#[derive(Clone, Debug)]
pub struct CreateLimitCurveReply {
    index: u32,
}

#[derive(Clone, Debug)]
pub struct RetrieveTadmDataReply {
    offsets: Vec<u16>,
    tadm_data: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetLimitCurveNamesReply {
    names: String,
}

#[derive(Clone, Debug)]
pub struct GetLimitCurveInfoReply {
    index: u32,
    lower_limits: u16,
    upper_limits: u16,
}

#[derive(Clone, Debug)]
pub struct GetTADMEnableReply {
    enable: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetZDefaultSpeedReply {
    speed: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct GetPotentiometerSettingsReply {
    gain: Vec<i16>,
    offset: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct ReadOnTheFlyDispenseErrorsReply {
    error: bool,
    results: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct GetChannelConfigurationReply {
    enabled: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct ZSeekObstaclePositionReply {
    z_position: Vec<i32>,
    obstacle_detected: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct ZSeekLldPositionReply {
    z_position: Vec<i32>,
    lld_detected: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetHomeSensorsReply {
    y_home: Vec<bool>,
    z_home: Vec<bool>,
    squeeze_home: Vec<bool>,
    dispense_home: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct GetCalibrationValuesReply {
    dispense_drive: Vec<i16>,
    squeeze_drive: Vec<i16>,
    pressure_lld: Vec<i16>,
    clot_detection: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetExtendedCalibrationValuesReply {
    tip_type_adjustment: Vec<i16>,
    pressure_conversion: Vec<i16>,
    aspirate_monitoring: Vec<i16>,
    z_bottom_search: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetCountersReply {
    tip_pickup_counter: Vec<u32>,
    tip_eject_counter: Vec<u32>,
    aspirate_counter: Vec<u32>,
    dispense_counter: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct GetPositionsReply {
    y_position: Vec<i32>,
    y_encoder_position: Vec<i32>,
    z_position: Vec<i32>,
    z_encoder_position: Vec<i32>,
    d_position: Vec<i32>,
    d_encoder_position: Vec<i32>,
    s_position: Vec<i32>,
    s_encoder_position: Vec<i32>,
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
