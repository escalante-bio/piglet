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

pub struct NimbusCoreChannel {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreChannel {
    // version 1.0

    pub fn new_1(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 272,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_2(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 273,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_3(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 274,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_4(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 275,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_5(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 276,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_6(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 277,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_7(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 278,
            },
            robot: robot.clone(),
        }
    }

    pub fn new_8(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 279,
            },
            robot: robot.clone(),
        }
    }

    pub async fn pickup_needle(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        volume: u32,
        length: u16,
        collet_check: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        volume.serialize(&mut args);
        length.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 7, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute_gripper(
        &self,
        position: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
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

    pub async fn z_move_absolute_speed(
        &self,
        position: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 30, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn read_on_the_fly_dispense_errors(
        &self,
    ) -> Result<ReadOnTheFlyDispenseErrorsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 94, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let status = String::deserialize(&mut stream)?;
        Ok(ReadOnTheFlyDispenseErrorsReply { status })
    }

    pub async fn squeeze_off(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 39, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_z_liquid_seek_speed(&self, velocity: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 103, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_y_position(&self) -> Result</* position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 48, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        Ok(position)
    }

    pub async fn is_tip_present(&self) -> Result</* present= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 45, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let present = bool::deserialize(&mut stream)?;
        Ok(present)
    }

    pub async fn get_extended_calibration_values(
        &self,
    ) -> Result<GetExtendedCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 110, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_type_adjustment = i16::deserialize(&mut stream)?;
        let pressure_conversion = i16::deserialize(&mut stream)?;
        let aspirate_monitoring = i16::deserialize(&mut stream)?;
        let z_bottom_search = i16::deserialize(&mut stream)?;
        Ok(GetExtendedCalibrationValuesReply {
            tip_type_adjustment,
            pressure_conversion,
            aspirate_monitoring,
            z_bottom_search,
        })
    }

    pub async fn paired_channel(&self, channel: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 42, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_limit_curve_names(&self) -> Result<GetLimitCurveNamesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 65, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let names = String::deserialize(&mut stream)?;
        Ok(GetLimitCurveNamesReply { names })
    }

    pub async fn z_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 25, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 17, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_current_dispenser_volume(&self) -> Result</* volume= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 97, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let volume = u32::deserialize(&mut stream)?;
        Ok(volume)
    }

    pub async fn get_liquid_height(&self) -> Result</* liquid_height= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 98, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let liquid_height = i32::deserialize(&mut stream)?;
        Ok(liquid_height)
    }

    pub async fn lld_off(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 41, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense_empty(&self) -> Result<(), Error> {
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

    pub async fn get_lld_status(&self) -> Result</* status= */ i16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 46, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let status = i16::deserialize(&mut stream)?;
        Ok(status)
    }

    pub async fn calibrate_tadm_offset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 90, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_absolute_2_speed(
        &self,
        position: i32,
        speed_switching_position: i32,
        velocity: u32,
        lower_section_velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        speed_switching_position.serialize(&mut args);
        velocity.serialize(&mut args);
        lower_section_velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 105, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn store_pressure_gain(&self, data_register: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        data_register.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 83, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn retrieve_tadm_data(&self) -> Result<RetrieveTadmDataReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 64, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tadm_data = Vec::<i16>::deserialize(&mut stream)?;
        Ok(RetrieveTadmDataReply { tadm_data })
    }

    pub async fn aspirate_capacitive_lld(
        &self,
        aspirate_mode: i16,
        liquid_seek_height: i32,
        submerge_depth: i32,
        follow_depth: i32,
        z_min_position: i32,
        clot_check_height: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        blowout_volume: u32,
        prewet_volume: u32,
        aspirate_volume: u32,
        transport_air_volume: u32,
        aspirate_speed: u32,
        settling_time: u32,
        mix_volume: u32,
        mix_cycles: u32,
        mix_position: i32,
        mix_follow_distance: i32,
        mix_speed: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        capacitive_lld_steepness: u32,
        capacitive_lld_offset: u32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        aspirate_mode.serialize(&mut args);
        liquid_seek_height.serialize(&mut args);
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
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        capacitive_lld_steepness.serialize(&mut args);
        capacitive_lld_offset.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 10, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense_lld(
        &self,
        liquid_seek_height: i32,
        submerge_depth: i32,
        follow_depth: i32,
        z_min_position: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        dispense_volume: u32,
        stop_back_volume: u32,
        transport_air_volume: u32,
        dispense_speed: u32,
        cutoff_speed: u32,
        settling_time: u32,
        mix_volume: u32,
        mix_cycles: u32,
        mix_position: i32,
        mix_follow_distance: i32,
        mix_speed: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        capacitive_lld_steepness: u32,
        capacitive_lld_offset: u32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        liquid_seek_height.serialize(&mut args);
        submerge_depth.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        dispense_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        mix_volume.serialize(&mut args);
        mix_cycles.serialize(&mut args);
        mix_position.serialize(&mut args);
        mix_follow_distance.serialize(&mut args);
        mix_speed.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        capacitive_lld_steepness.serialize(&mut args);
        capacitive_lld_offset.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn erase_limit_curves(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 61, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_position_reset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 54, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_limit_curve_info(
        &self,
        name: String,
    ) -> Result<GetLimitCurveInfoReply, Error> {
        let mut args = BytesMut::new();
        name.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 66, args.freeze())
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

    pub async fn drop_tip(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 8, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 34, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_relative(&self, distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 18, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn receive_no_wait(&self) -> Result</* response= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let response = String::deserialize(&mut stream)?;
        Ok(response)
    }

    pub async fn calibrate_squeeze(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 56, args.freeze())
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

    pub async fn squeeze_on(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 38, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_calibration_values(&self) -> Result<GetCalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 109, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let dispense_drive = i16::deserialize(&mut stream)?;
        let squeeze_drive = i16::deserialize(&mut stream)?;
        let pressure_lld = i16::deserialize(&mut stream)?;
        let clot_detection = i16::deserialize(&mut stream)?;
        Ok(GetCalibrationValuesReply {
            dispense_drive,
            squeeze_drive,
            pressure_lld,
            clot_detection,
        })
    }

    pub async fn calibrate_dispense_check(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        volume: u32,
        collet_check: i16,
    ) -> Result</* steps= */ i32, Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 58, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let steps = i32::deserialize(&mut stream)?;
        Ok(steps)
    }

    pub async fn synchronize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 43, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_counters(
        &self,
        tip_pickup_counter: u32,
        tip_eject_counter: u32,
        aspirate_counter: u32,
        dispense_counter: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tip_pickup_counter.serialize(&mut args);
        tip_eject_counter.serialize(&mut args);
        aspirate_counter.serialize(&mut args);
        dispense_counter.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 113, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_squeeze_position(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 102, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_offset(&self, offset: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        offset.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 81, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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

    pub async fn set_configuration(&self, index: i16, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        index.serialize(&mut args);
        enable.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 87, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_touchoff(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 60, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_y_home_offset(&self, home_offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        home_offset.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 50, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn reset_tadm_fifo(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 63, args.freeze())
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

    pub async fn store_pressure_offset(&self, data_register: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        data_register.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 84, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_y_home_offset(&self) -> Result</* home_offset= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 49, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let home_offset = i32::deserialize(&mut stream)?;
        Ok(home_offset)
    }

    pub async fn z_seek_obstacle_position(
        &self,
        z_position: i32,
        z_min_position: i32,
        z_final_position: i32,
        z_velocity: u32,
    ) -> Result<ZSeekObstaclePositionReply, Error> {
        let mut args = BytesMut::new();
        z_position.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final_position.serialize(&mut args);
        z_velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 99, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        let obstacle_detected = bool::deserialize(&mut stream)?;
        Ok(ZSeekObstaclePositionReply {
            position,
            obstacle_detected,
        })
    }

    pub async fn z_move_absolute_gripper(
        &self,
        position: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        acceleration.serialize(&mut args);
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

    pub async fn disable_adc(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 72, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn send(&self, request: String) -> Result<(), Error> {
        let mut args = BytesMut::new();
        request.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 2, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn measure_lld_frequency(
        &self,
        seconds: u16,
    ) -> Result</* detects_per_minute= */ u16, Error> {
        let mut args = BytesMut::new();
        seconds.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 114, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let detects_per_minute = u16::deserialize(&mut stream)?;
        Ok(detects_per_minute)
    }

    pub async fn get_configuration(&self, index: i16) -> Result<GetConfigurationReply, Error> {
        let mut args = BytesMut::new();
        index.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 86, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enabled = bool::deserialize(&mut stream)?;
        Ok(GetConfigurationReply { enabled })
    }

    pub async fn y_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 20, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn receive(&self, command_timeout: i32) -> Result</* response= */ String, Error> {
        let mut args = BytesMut::new();
        command_timeout.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 4, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let response = String::deserialize(&mut stream)?;
        Ok(response)
    }

    pub async fn y_move_relative_speed(
        &self,
        distance: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 19, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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

    pub async fn z_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 29, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_safe(&self) -> Result<(), Error> {
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

    pub async fn get_counters(&self) -> Result<GetCountersReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 111, args.freeze())
            .await?;
        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let tip_pickup_counter = u32::deserialize(&mut stream)?;
        let tip_eject_counter = u32::deserialize(&mut stream)?;
        let aspirate_counter = u32::deserialize(&mut stream)?;
        let dispense_counter = u32::deserialize(&mut stream)?;
        Ok(GetCountersReply {
            tip_pickup_counter,
            tip_eject_counter,
            aspirate_counter,
            dispense_counter,
        })
    }

    pub async fn get_y_resolution(&self) -> Result</* resolution= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 77, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let resolution = i32::deserialize(&mut stream)?;
        Ok(resolution)
    }

    pub async fn get_positions(&self) -> Result<GetPositionsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 115, args.freeze())
            .await?;
        if count != 8 {
            return Err(ConnectionError(anyhow!("Expected 8 values, not {}", count)));
        }
        let y_position = i32::deserialize(&mut stream)?;
        let y_encoder_position = i32::deserialize(&mut stream)?;
        let z_position = i32::deserialize(&mut stream)?;
        let z_encoder_position = i32::deserialize(&mut stream)?;
        let d_position = i32::deserialize(&mut stream)?;
        let d_encoder_position = i32::deserialize(&mut stream)?;
        let s_position = i32::deserialize(&mut stream)?;
        let s_encoder_position = i32::deserialize(&mut stream)?;
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

    pub async fn prepare_on_the_fly_dispense(
        &self,
        volume: u32,
        stop_back_volume: u32,
        transport_air_volume: u32,
        dispense_speed: u32,
        cutoff_speed: u32,
        dispense_pattern: String,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        dispense_pattern.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 91, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispenser_dispense(
        &self,
        volume: u32,
        stop_back_volume: u32,
        dispense_speed: u32,
        cutoff_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 96, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_tadm_enable(&self) -> Result</* enable= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 68, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = bool::deserialize(&mut stream)?;
        Ok(enable)
    }

    pub async fn pressure_shift(&self, shift: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        shift.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 82, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_sensor_status(&self) -> Result</* status= */ i16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 47, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let status = i16::deserialize(&mut stream)?;
        Ok(status)
    }

    pub async fn calibrate_lld(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 55, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn command(
        &self,
        command_timeout: i32,
        request: String,
    ) -> Result</* response= */ String, Error> {
        let mut args = BytesMut::new();
        command_timeout.serialize(&mut args);
        request.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 1, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let response = String::deserialize(&mut stream)?;
        Ok(response)
    }

    pub async fn z_move_traverse(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 31, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn last_command(&self) -> Result</* request= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 5, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let request = String::deserialize(&mut stream)?;
        Ok(request)
    }

    pub async fn z_move_relative(&self, distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 27, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn enable_mad(
        &self,
        pressure_threshold: u16,
        minimum_pressure_difference: u16,
        maximum_pressure_difference: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        pressure_threshold.serialize(&mut args);
        minimum_pressure_difference.serialize(&mut args);
        maximum_pressure_difference.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 69, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_technical_info(&self) -> Result</* channel_info= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 112, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channel_info = String::deserialize(&mut stream)?;
        Ok(channel_info)
    }

    pub async fn dispenser_aspirate(&self, volume: u32, aspirate_speed: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        volume.serialize(&mut args);
        aspirate_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 95, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense(
        &self,
        dispense_height: i32,
        follow_depth: i32,
        z_min_position: i32,
        air_height: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        dispense_volume: u32,
        stop_back_volume: u32,
        transport_air_volume: u32,
        dispense_speed: u32,
        cutoff_speed: u32,
        settling_time: u32,
        mix_volume: u32,
        mix_cycles: u32,
        mix_position: i32,
        mix_follow_distance: i32,
        mix_speed: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        bottom_search: bool,
        dispense_offset: i32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        dispense_height.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        air_height.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        dispense_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        mix_volume.serialize(&mut args);
        mix_cycles.serialize(&mut args);
        mix_position.serialize(&mut args);
        mix_follow_distance.serialize(&mut args);
        mix_speed.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        bottom_search.serialize(&mut args);
        dispense_offset.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 14, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_pressure(&self) -> Result</* pressure= */ i16, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 76, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let pressure = i16::deserialize(&mut stream)?;
        Ok(pressure)
    }

    pub async fn stop_on_the_fly_dispense(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 93, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_servo_off(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 106, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn disable_mad(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 70, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_negative_stall(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 23, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn lld_on(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 40, args.freeze())
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

    pub async fn calibrate_squeeze_check_torque(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 101, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_tip_height(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        volume: u32,
        collet_check: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 57, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn squeeze_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 37, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_dispense_check_data(
        &self,
        direction: i16,
        index: i32,
    ) -> Result</* result= */ String, Error> {
        let mut args = BytesMut::new();
        direction.serialize(&mut args);
        index.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 59, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let result = String::deserialize(&mut stream)?;
        Ok(result)
    }

    pub async fn reset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 116, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_lld(
        &self,
        position: i32,
        min_position: i32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        min_position.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 32, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_relative_speed(
        &self,
        distance: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        acceleration.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 28, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense_move_relative(&self, distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 36, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_tip_no_z_final(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_z_default_speed(&self, speed: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 73, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_tadm_enable(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 67, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn create_limit_curve(
        &self,
        name: String,
        lower_limit_x: Vec<u16>,
        lower_limit_y: Vec<i16>,
        upper_limit_x: Vec<u16>,
        upper_limit_y: Vec<i16>,
    ) -> Result<CreateLimitCurveReply, Error> {
        let mut args = BytesMut::new();
        name.serialize(&mut args);
        lower_limit_x.serialize(&mut args);
        lower_limit_y.serialize(&mut args);
        upper_limit_x.serialize(&mut args);
        upper_limit_y.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 62, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let index = u32::deserialize(&mut stream)?;
        Ok(CreateLimitCurveReply { index })
    }

    pub async fn get_z_liquid_seek_speed(&self) -> Result</* velocity= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 104, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let velocity = u32::deserialize(&mut stream)?;
        Ok(velocity)
    }

    pub async fn enable_adc(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 71, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_servo_off(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 107, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn empty_tip(
        &self,
        dispense_height: i32,
        follow_depth: i32,
        z_min_position: i32,
        air_height: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        transport_air_volume: u32,
        dispense_speed: u32,
        cutoff_speed: u32,
        settling_time: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        dispense_height.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        air_height.serialize(&mut args);
        z_final.serialize(&mut args);
        liquid_exit_speed.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        settling_time.serialize(&mut args);
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 15, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_pressure_sensor_ad(&self) -> Result<GetPressureSensorADReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 89, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let pressure_lld = i16::deserialize(&mut stream)?;
        let tadm = i16::deserialize(&mut stream)?;
        Ok(GetPressureSensorADReply { pressure_lld, tadm })
    }

    pub async fn is_initialized(&self) -> Result</* initialized= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 44, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(initialized)
    }

    pub async fn pressure_mode(&self, mode: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        mode.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 79, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate_pressure_lld(
        &self,
        aspirate_mode: i16,
        liquid_seek_height: i32,
        submerge_depth: i32,
        follow_depth: i32,
        z_min_position: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        blowout_volume: u32,
        prewet_volume: u32,
        aspirate_volume: u32,
        transport_air_volume: u32,
        aspirate_speed: u32,
        settling_time: u32,
        mix_volume: u32,
        mix_cycles: u32,
        mix_position: i32,
        mix_follow_distance: i32,
        mix_speed: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        lld_mode: i16,
        capacitive_lld_steepness: u32,
        capacitive_lld_offset: u32,
        pressure_lld_steepness: u32,
        pressure_lld_offset: u32,
        seek_speed: u32,
        lld_height_difference: i32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
        tip_is_filtered: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        aspirate_mode.serialize(&mut args);
        liquid_seek_height.serialize(&mut args);
        submerge_depth.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
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
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        lld_mode.serialize(&mut args);
        capacitive_lld_steepness.serialize(&mut args);
        capacitive_lld_offset.serialize(&mut args);
        pressure_lld_steepness.serialize(&mut args);
        pressure_lld_offset.serialize(&mut args);
        seek_speed.serialize(&mut args);
        lld_height_difference.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        tip_is_filtered.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_potentiometer_data(&self) -> Result<GetPotentiometerDataReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 88, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let gain = i16::deserialize(&mut stream)?;
        let offset = i16::deserialize(&mut stream)?;
        Ok(GetPotentiometerDataReply { gain, offset })
    }

    pub async fn pickup_tip(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        volume: u32,
        collet_check: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 6, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_seek_lld(&self, position: i32, velocity: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 22, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn save_counters(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 108, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pressure_gain(&self, gain: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        gain.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 80, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_obstacle(
        &self,
        z_position: i32,
        z_min_position: i32,
        z_final_position: i32,
        z_velocity: u32,
    ) -> Result</* obstacle_detected= */ bool, Error> {
        let mut args = BytesMut::new();
        z_position.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final_position.serialize(&mut args);
        z_velocity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 75, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let obstacle_detected = bool::deserialize(&mut stream)?;
        Ok(obstacle_detected)
    }

    pub async fn set_z_home_offset(&self, home_offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        home_offset.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 53, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate(
        &self,
        aspirate_mode: i16,
        aspirate_height: i32,
        follow_depth: i32,
        z_min_position: i32,
        air_height: i32,
        z_final: i32,
        liquid_exit_speed: u32,
        blowout_volume: u32,
        prewet_volume: u32,
        aspirate_volume: u32,
        transport_air_volume: u32,
        aspirate_speed: u32,
        settling_time: u32,
        mix_volume: u32,
        mix_cycles: u32,
        mix_position: i32,
        mix_follow_distance: i32,
        mix_speed: u32,
        tube_section_height: i32,
        tube_section_ratio: i32,
        bottom_search: bool,
        aspirate_offset: i32,
        tadm_enabled: bool,
        limit_curve_index: u32,
        recording_mode: u16,
        measurement_id: String,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        aspirate_mode.serialize(&mut args);
        aspirate_height.serialize(&mut args);
        follow_depth.serialize(&mut args);
        z_min_position.serialize(&mut args);
        air_height.serialize(&mut args);
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
        tube_section_height.serialize(&mut args);
        tube_section_ratio.serialize(&mut args);
        bottom_search.serialize(&mut args);
        aspirate_offset.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        measurement_id.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 12, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_home_offset(&self) -> Result</* home_offset= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 52, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let home_offset = i32::deserialize(&mut stream)?;
        Ok(home_offset)
    }

    pub async fn start_on_the_fly_dispense(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 92, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_pre_initialize(&self) -> Result<(), Error> {
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

    pub async fn get_z_default_speed(&self) -> Result</* speed= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 74, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let speed = i32::deserialize(&mut stream)?;
        Ok(speed)
    }

    pub async fn y_move_absolute_speed(
        &self,
        position: i32,
        acceleration: u32,
        velocity: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
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

    pub async fn leak_check(
        &self,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        tip_volume: u32,
        collet_check: i16,
        time: u32,
        test_type: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_volume.serialize(&mut args);
        collet_check.serialize(&mut args);
        time.serialize(&mut args);
        test_type.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 85, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_y_resolution(&self, resolution: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        resolution.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 78, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_position(&self) -> Result</* position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 51, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        Ok(position)
    }

    pub async fn z_seek_lld_position(
        &self,
        z_position: i32,
        z_min_position: i32,
        z_final_position: i32,
        z_velocity: u32,
        capacitive_lld_steepness: u32,
        capacitive_lld_offset: u32,
    ) -> Result<ZSeekLldPositionReply, Error> {
        let mut args = BytesMut::new();
        z_position.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final_position.serialize(&mut args);
        z_velocity.serialize(&mut args);
        capacitive_lld_steepness.serialize(&mut args);
        capacitive_lld_offset.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 100, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        let lld_detected = bool::deserialize(&mut stream)?;
        Ok(ZSeekLldPositionReply {
            position,
            lld_detected,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ColletChecks {
    ColletCheckNone = 0,
    ColletCheckLow = 1,
    ColletCheckStandard = 2,
    ColletCheckHigh = 3,
    ColletCheck384 = 4,
    ColletCheck5ml = 5,
}

impl TryFrom<i32> for ColletChecks {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ColletChecks::ColletCheckNone),
            1 => Ok(ColletChecks::ColletCheckLow),
            2 => Ok(ColletChecks::ColletCheckStandard),
            3 => Ok(ColletChecks::ColletCheckHigh),
            4 => Ok(ColletChecks::ColletCheck384),
            5 => Ok(ColletChecks::ColletCheck5ml),

            _ => Err(ConnectionError(anyhow!("Unknown ColletChecks value {}", v))),
        }
    }
}

impl PigletCodec for ColletChecks {
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

impl PigletCodec for MVec<Vec<ColletChecks>> {
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
pub enum TadmModes {
    TadmModeNone = 0,
    TadmModeError = 1,
    TadmModeAll = 2,
}

impl TryFrom<i32> for TadmModes {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(TadmModes::TadmModeNone),
            1 => Ok(TadmModes::TadmModeError),
            2 => Ok(TadmModes::TadmModeAll),

            _ => Err(ConnectionError(anyhow!("Unknown TadmModes value {}", v))),
        }
    }
}

impl PigletCodec for TadmModes {
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

impl PigletCodec for MVec<Vec<TadmModes>> {
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
pub enum ConfigurationIndexes {
    ConfigIndexInvalid = 0,
    ConfigIndexTip = 1,
    ConfigIndexPlld = 2,
    ConfigIndexClld = 3,
    ConfigIndexClot = 4,
}

impl TryFrom<i32> for ConfigurationIndexes {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ConfigurationIndexes::ConfigIndexInvalid),
            1 => Ok(ConfigurationIndexes::ConfigIndexTip),
            2 => Ok(ConfigurationIndexes::ConfigIndexPlld),
            3 => Ok(ConfigurationIndexes::ConfigIndexClld),
            4 => Ok(ConfigurationIndexes::ConfigIndexClot),

            _ => Err(ConnectionError(anyhow!(
                "Unknown ConfigurationIndexes value {}",
                v
            ))),
        }
    }
}

impl PigletCodec for ConfigurationIndexes {
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

impl PigletCodec for MVec<Vec<ConfigurationIndexes>> {
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
pub struct ReadOnTheFlyDispenseErrorsReply {
    status: String,
}

#[derive(Clone, Debug)]
pub struct GetExtendedCalibrationValuesReply {
    tip_type_adjustment: i16,
    pressure_conversion: i16,
    aspirate_monitoring: i16,
    z_bottom_search: i16,
}

#[derive(Clone, Debug)]
pub struct GetLimitCurveNamesReply {
    names: String,
}

#[derive(Clone, Debug)]
pub struct RetrieveTadmDataReply {
    tadm_data: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetLimitCurveInfoReply {
    index: u32,
    lower_limits: u16,
    upper_limits: u16,
}

#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
}

#[derive(Clone, Debug)]
pub struct GetCalibrationValuesReply {
    dispense_drive: i16,
    squeeze_drive: i16,
    pressure_lld: i16,
    clot_detection: i16,
}

#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    enumeration_names: Vec<String>,
    number_enumeration_values: Vec<u32>,
    enumeration_values: Vec<i16>,
    enumeration_value_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
}

#[derive(Clone, Debug)]
pub struct ZSeekObstaclePositionReply {
    position: i32,
    obstacle_detected: bool,
}

#[derive(Clone, Debug)]
pub struct GetConfigurationReply {
    enabled: bool,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GetCountersReply {
    tip_pickup_counter: u32,
    tip_eject_counter: u32,
    aspirate_counter: u32,
    dispense_counter: u32,
}

#[derive(Clone, Debug)]
pub struct GetPositionsReply {
    y_position: i32,
    y_encoder_position: i32,
    z_position: i32,
    z_encoder_position: i32,
    d_position: i32,
    d_encoder_position: i32,
    s_position: i32,
    s_encoder_position: i32,
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
pub struct InterfaceDescriptorsReply {
    interface_ids: Vec<u8>,
    interface_descriptors: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CreateLimitCurveReply {
    index: u32,
}

#[derive(Clone, Debug)]
pub struct GetPressureSensorADReply {
    pressure_lld: i16,
    tadm: i16,
}

#[derive(Clone, Debug)]
pub struct GetPotentiometerDataReply {
    gain: i16,
    offset: i16,
}

#[derive(Clone, Debug)]
pub struct ZSeekLldPositionReply {
    position: i32,
    lld_detected: bool,
}
