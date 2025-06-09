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

pub struct NimbusCorePipette {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCorePipette {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 257,
            },
            robot: robot.clone(),
        }
    }

    pub async fn move_relative_x(&self, x_distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 27, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate_1(
        &self,
        aspirate_type: Vec<i16>,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
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
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
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
            .act(&self.address, 1, 3, 6, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn aspirate_2(
        &self,
        aspirate_type: Vec<i16>,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
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
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
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
            .act(&self.address, 1, 3, 6, args.freeze())
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

    pub async fn pickup_gripper_tool(
        &self,
        x_position: i32,
        y_position_1_st_channel: i32,
        y_position_2_nd_channel: i32,
        traverse_height: i32,
        z_start_position: i32,
        z_stop_position: i32,
        tip_type: u16,
        first_channel_number: u16,
        second_channel_number: u16,
        tool_width: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position_1_st_channel.serialize(&mut args);
        y_position_2_nd_channel.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        tip_type.serialize(&mut args);
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        tool_width.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 9, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn multiple_dispense_transport_air(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        x_well_distance: i32,
        x_well_width: u32,
        x_number_of_wells: u16,
        x_velocity_scale: u16,
        y_position: Vec<i32>,
        y_well_distance: Vec<i32>,
        volume: Vec<u32>,
        tip_air_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        traverse_height: i32,
        dispense_height: Vec<i32>,
        z_final: i32,
        dispense_map: Vec<u16>,
        transport_air_volume: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        x_well_distance.serialize(&mut args);
        x_well_width.serialize(&mut args);
        x_number_of_wells.serialize(&mut args);
        x_velocity_scale.serialize(&mut args);
        y_position.serialize(&mut args);
        y_well_distance.serialize(&mut args);
        volume.serialize(&mut args);
        tip_air_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        traverse_height.serialize(&mut args);
        dispense_height.serialize(&mut args);
        z_final.serialize(&mut args);
        dispense_map.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 71, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_plate_suction(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        pressure_differential: u16,
        traverse_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        pressure_differential.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 48, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_x(&self, x_position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 26, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_initialized(&self) -> Result</* initialized= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 15, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(initialized)
    }

    pub async fn reset_tadm_fifo(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 35, args.freeze())
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
            .act(&self.address, 1, 3, 43, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_obstacle_position(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        obstacle_seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
    ) -> Result<ZSeekObstaclePositionReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        obstacle_seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 68, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let z_position = Vec::<i16>::deserialize(&mut stream)?;
        let obstacle_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(ZSeekObstaclePositionReply {
            z_position,
            obstacle_detected,
        })
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

    pub async fn dispense(
        &self,
        dispense_type: Vec<i16>,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
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
        touch_off_distance: i32,
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
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
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
        touch_off_distance.serialize(&mut args);
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
            .act(&self.address, 1, 3, 7, args.freeze())
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

    pub async fn is_core_gripper_tool_held(&self) -> Result<IsCoreGripperToolHeldReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 17, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let gripped = bool::deserialize(&mut stream)?;
        let tip_type = Vec::<u16>::deserialize(&mut stream)?;
        Ok(IsCoreGripperToolHeldReply { gripped, tip_type })
    }

    pub async fn get_remaining_channels(&self) -> Result<GetRemainingChannelsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 50, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channels = Vec::<u16>::deserialize(&mut stream)?;
        Ok(GetRemainingChannelsReply { channels })
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
            .act(&self.address, 1, 3, 34, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let index = u32::deserialize(&mut stream)?;
        Ok(CreateLimitCurveReply { index })
    }

    pub async fn disable_mad(&self, tips_used: Vec<u16>) -> Result<(), Error> {
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

    pub async fn set_performance_parameters(
        &self,
        z_speed: i32,
        time: u16,
        time_limit: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_speed.serialize(&mut args);
        time.serialize(&mut args);
        time_limit.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 51, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_dispenser(
        &self,
        tips_used: Vec<u16>,
        volumes: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        volumes.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 61, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_seek_lld_position(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
        capacitive_lld_sensitivity: Vec<i16>,
    ) -> Result<ZSeekLldPositionReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        capacitive_lld_sensitivity.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 69, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let z_position = Vec::<i16>::deserialize(&mut stream)?;
        let lld_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(ZSeekLldPositionReply {
            z_position,
            lld_detected,
        })
    }

    pub async fn get_limit_curve_names(
        &self,
        channel: u16,
    ) -> Result<GetLimitCurveNamesReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 37, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let names = String::deserialize(&mut stream)?;
        Ok(GetLimitCurveNamesReply { names })
    }

    pub async fn initialize_dispenser(&self, tips_used: Vec<u16>) -> Result<(), Error> {
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

    pub async fn drop_plate_suction_2(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        traverse_height: i32,
        z_drop_height: i32,
        z_press_distance: i32,
        z_lift_distance: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_drop_height.serialize(&mut args);
        z_press_distance.serialize(&mut args);
        z_lift_distance.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 77, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
            .act(&self.address, 1, 3, 80, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_z_2_speed(
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
            .act(&self.address, 1, 3, 75, args.freeze())
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

    pub async fn squeeze_on(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 58, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_liquid_height(&self) -> Result</* liquid_height= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 65, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let liquid_height = Vec::<i16>::deserialize(&mut stream)?;
        Ok(liquid_height)
    }

    pub async fn get_position(&self) -> Result<GetPositionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 20, args.freeze())
            .await?;
        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let x_position = i32::deserialize(&mut stream)?;
        let y_position = Vec::<i16>::deserialize(&mut stream)?;
        let z_position = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetPositionReply {
            x_position,
            y_position,
            z_position,
        })
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
            .act(&self.address, 1, 3, 73, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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

    pub async fn drop_tips_roll(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
        roll_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        roll_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 82, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_tips(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        z_final: Vec<i32>,
        default_waste: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        default_waste.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 5, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_x(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 53, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
            .act(&self.address, 1, 3, 67, args.freeze())
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
            .act(&self.address, 1, 3, 44, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_y(
        &self,
        tips_used: Vec<u16>,
        y_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 29, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_z(
        &self,
        tips_used: Vec<u16>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 30, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_y(
        &self,
        tips_used: Vec<u16>,
        y_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        y_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 28, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
            .act(&self.address, 1, 3, 41, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_z(
        &self,
        tips_used: Vec<u16>,
        z_distance: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_distance.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 31, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_z(&self, tips_used: Vec<u16>) -> Result<(), Error> {
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

    pub async fn is_tip_present(&self) -> Result</* tip_present= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 16, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_present = Vec::<i16>::deserialize(&mut stream)?;
        Ok(tip_present)
    }

    pub async fn pickup_plate_suction(
        &self,
        x_position: i32,
        y_plate_center_position: i32,
        y_suction_offset: i32,
        blowout_volume: u32,
        suction_volume: u32,
        pressure_differential: u16,
        traverse_height: i32,
        z_grip_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<PickupPlateSuctionReply, Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        y_suction_offset.serialize(&mut args);
        blowout_volume.serialize(&mut args);
        suction_volume.serialize(&mut args);
        pressure_differential.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_grip_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 46, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let first_pressure_change = i16::deserialize(&mut stream)?;
        let second_pressure_change = i16::deserialize(&mut stream)?;
        Ok(PickupPlateSuctionReply {
            first_pressure_change,
            second_pressure_change,
        })
    }

    pub async fn release_plate(
        &self,
        first_channel_number: u16,
        second_channel_number: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 14, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_x_interlock_active(&self) -> Result</* active= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 85, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let active = bool::deserialize(&mut stream)?;
        Ok(active)
    }

    pub async fn move_to_position_via_lane(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        y_position: Vec<i32>,
        traverse_height: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 24, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_tip_and_needle_types(&self) -> Result</* tip_type= */ Vec<u16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 19, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_type = Vec::<u16>::deserialize(&mut stream)?;
        Ok(tip_type)
    }

    pub async fn retrieve_tadm_data(
        &self,
        tips_used: Vec<u16>,
    ) -> Result<RetrieveTadmDataReply, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 36, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let offsets = Vec::<u16>::deserialize(&mut stream)?;
        let tadm_data = Vec::<i16>::deserialize(&mut stream)?;
        Ok(RetrieveTadmDataReply { offsets, tadm_data })
    }

    pub async fn on_the_fly_dispense(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        x_well_distance: i32,
        x_well_width: u32,
        x_number_of_wells: u16,
        x_velocity_scale: u16,
        y_position: Vec<i32>,
        y_well_distance: Vec<i32>,
        volume: Vec<u32>,
        tip_air_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        tadm_enabled: Vec<i16>,
        limit_curve_index: Vec<u32>,
        recording_mode: u16,
        traverse_height: i32,
        dispense_height: Vec<i32>,
        z_final: i32,
        dispense_map: Vec<u16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        x_well_distance.serialize(&mut args);
        x_well_width.serialize(&mut args);
        x_number_of_wells.serialize(&mut args);
        x_velocity_scale.serialize(&mut args);
        y_position.serialize(&mut args);
        y_well_distance.serialize(&mut args);
        volume.serialize(&mut args);
        tip_air_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        traverse_height.serialize(&mut args);
        dispense_height.serialize(&mut args);
        z_final.serialize(&mut args);
        dispense_map.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 63, args.freeze())
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

    pub async fn z_seek_obstacle(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        obstacle_seek_height: Vec<i32>,
        z_min_position: Vec<i32>,
        z_final: i32,
        seek_speed: Vec<u32>,
    ) -> Result</* obstacle_detected= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        obstacle_seek_height.serialize(&mut args);
        z_min_position.serialize(&mut args);
        z_final.serialize(&mut args);
        seek_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 49, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let obstacle_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(obstacle_detected)
    }

    pub async fn move_over(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        y_position: Vec<i32>,
        traverse_height: i32,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
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

    pub async fn get_z_liquid_seek_speed(&self) -> Result</* seek_speeds= */ Vec<u32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 74, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let seek_speeds = Vec::<u32>::deserialize(&mut stream)?;
        Ok(seek_speeds)
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
            .act(&self.address, 1, 0, 38, args.freeze())
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

    pub async fn get_tadm_enable(&self) -> Result<GetTADMEnableReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 40, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetTADMEnableReply { enable })
    }

    pub async fn reset_performance_parameters(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 52, args.freeze())
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
            .act(&self.address, 1, 3, 83, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
            .act(&self.address, 1, 0, 66, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enabled = Vec::<bool>::deserialize(&mut stream)?;
        Ok(GetChannelConfigurationReply { enabled })
    }

    pub async fn pickup_plate_suction_2(
        &self,
        x_position: i32,
        y_plate_center_position: i32,
        y_suction_offset: i32,
        blowout_volume: u32,
        suction_volume: u32,
        traverse_height: i32,
        z_grip_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        y_suction_offset.serialize(&mut args);
        blowout_volume.serialize(&mut args);
        suction_volume.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_grip_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 76, args.freeze())
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
            .act(&self.address, 1, 3, 21, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn multiple_dispense(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        x_well_distance: i32,
        x_well_width: u32,
        x_number_of_wells: u16,
        x_velocity_scale: u16,
        y_position: Vec<i32>,
        y_well_distance: Vec<i32>,
        volume: Vec<u32>,
        tip_air_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        traverse_height: i32,
        dispense_height: Vec<i32>,
        z_final: i32,
        dispense_map: Vec<u16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        x_well_distance.serialize(&mut args);
        x_well_width.serialize(&mut args);
        x_number_of_wells.serialize(&mut args);
        x_velocity_scale.serialize(&mut args);
        y_position.serialize(&mut args);
        y_well_distance.serialize(&mut args);
        volume.serialize(&mut args);
        tip_air_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        traverse_height.serialize(&mut args);
        dispense_height.serialize(&mut args);
        z_final.serialize(&mut args);
        dispense_map.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 62, args.freeze())
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
            .act(&self.address, 1, 3, 59, args.freeze())
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

    pub async fn pickup_plate(
        &self,
        x_position: i32,
        y_plate_center_position: i32,
        y_plate_width: u32,
        y_open_position: i32,
        y_grip_speed: u32,
        y_grip_strength: u32,
        traverse_height: i32,
        z_grip_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        y_plate_width.serialize(&mut args);
        y_open_position.serialize(&mut args);
        y_grip_speed.serialize(&mut args);
        y_grip_strength.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_grip_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 11, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_plate_suction(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        pressure_differential: u16,
        traverse_height: i32,
        z_drop_height: i32,
        z_press_distance: i32,
        z_lift_distance: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<DropPlateSuctionReply, Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        pressure_differential.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_drop_height.serialize(&mut args);
        z_press_distance.serialize(&mut args);
        z_lift_distance.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 47, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let first_pressure_change = i16::deserialize(&mut stream)?;
        let second_pressure_change = i16::deserialize(&mut stream)?;
        Ok(DropPlateSuctionReply {
            first_pressure_change,
            second_pressure_change,
        })
    }

    pub async fn move_plate(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        traverse_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 13, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_squeeze(&self, tips_used: Vec<u16>) -> Result<(), Error> {
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

    pub async fn on_the_fly_dispense_transport_air(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        x_well_distance: i32,
        x_well_width: u32,
        x_number_of_wells: u16,
        x_velocity_scale: u16,
        y_position: Vec<i32>,
        y_well_distance: Vec<i32>,
        volume: Vec<u32>,
        tip_air_volume: Vec<u32>,
        stop_back_volume: Vec<u32>,
        dispense_speed: Vec<u32>,
        cutoff_speed: Vec<u32>,
        tadm_enabled: Vec<i16>,
        limit_curve_index: Vec<u32>,
        recording_mode: u16,
        traverse_height: i32,
        dispense_height: Vec<i32>,
        z_final: i32,
        dispense_map: Vec<u16>,
        transport_air_volume: Vec<u32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        x_well_distance.serialize(&mut args);
        x_well_width.serialize(&mut args);
        x_number_of_wells.serialize(&mut args);
        x_velocity_scale.serialize(&mut args);
        y_position.serialize(&mut args);
        y_well_distance.serialize(&mut args);
        volume.serialize(&mut args);
        tip_air_volume.serialize(&mut args);
        stop_back_volume.serialize(&mut args);
        dispense_speed.serialize(&mut args);
        cutoff_speed.serialize(&mut args);
        tadm_enabled.serialize(&mut args);
        limit_curve_index.serialize(&mut args);
        recording_mode.serialize(&mut args);
        traverse_height.serialize(&mut args);
        dispense_height.serialize(&mut args);
        z_final.serialize(&mut args);
        dispense_map.serialize(&mut args);
        transport_air_volume.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 72, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pickup_tips(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
        z_start_position: Vec<i32>,
        z_stop_position: Vec<i32>,
        tip_type: Vec<u16>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        tip_type.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 4, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_to_position(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        y_position: Vec<i32>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 23, args.freeze())
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
            .act(&self.address, 1, 3, 81, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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

    pub async fn erase_limit_curves(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 33, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_traverse_z(
        &self,
        tips_used: Vec<u16>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 45, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn dispense_initialize_to_waste(
        &self,
        tips_used: Vec<u16>,
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        z_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 70, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_plate(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        y_open_position: i32,
        traverse_height: i32,
        z_drop_height: i32,
        z_press_distance: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        y_open_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_drop_height.serialize(&mut args);
        z_press_distance.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 12, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_core_gripper_plate_gripped(&self) -> Result</* gripped= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 18, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let gripped = bool::deserialize(&mut stream)?;
        Ok(gripped)
    }

    pub async fn move_plate_suction_2(
        &self,
        x_position: i32,
        x_acceleration: u32,
        y_plate_center_position: i32,
        traverse_height: i32,
        z_final: i32,
        z_speed: u32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        x_acceleration.serialize(&mut args);
        y_plate_center_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_final.serialize(&mut args);
        z_speed.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 78, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn check_plate_suction(
        &self,
        first_channel_number: u16,
        second_channel_number: u16,
        suction_volume: u32,
    ) -> Result<CheckPlateSuctionReply, Error> {
        let mut args = BytesMut::new();
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        suction_volume.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 0, 79, args.freeze())
            .await?;
        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let first_channel_has_plate = i16::deserialize(&mut stream)?;
        let second_channel_has_plate = i16::deserialize(&mut stream)?;
        Ok(CheckPlateSuctionReply {
            first_channel_has_plate,
            second_channel_has_plate,
        })
    }

    pub async fn drop_gripper_tool(
        &self,
        x_position: i32,
        y_position_1_st_channel: i32,
        y_position_2_nd_channel: i32,
        traverse_height: i32,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        first_channel_number: u16,
        second_channel_number: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position_1_st_channel.serialize(&mut args);
        y_position_2_nd_channel.serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 10, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
            .act(&self.address, 1, 3, 39, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn preinitialize_smart(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 32, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_y(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 54, args.freeze())
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
            .act(&self.address, 1, 0, 64, args.freeze())
            .await?;
        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let volume = Vec::<u32>::deserialize(&mut stream)?;
        Ok(volume)
    }

    pub async fn tip_and_needle_definition(
        &self,
        tip_type: u16,
        volume: u32,
        length: u16,
        collar_type: i8,
        filtered: bool,
        needle: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tip_type.serialize(&mut args);
        volume.serialize(&mut args);
        length.serialize(&mut args);
        collar_type.serialize(&mut args);
        filtered.serialize(&mut args);
        needle.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 3, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn empty_dispenser(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 60, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_xy(
        &self,
        tips_used: Vec<u16>,
        x_position: i32,
        y_position: Vec<i32>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        let (count, mut stream) = self
            .robot
            .act(&self.address, 1, 3, 25, args.freeze())
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
        x_position: Vec<i32>,
        y_position: Vec<i32>,
        traverse_height: i32,
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
        dispense_offset: Vec<i32>,
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
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        traverse_height.serialize(&mut args);
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
        dispense_offset.serialize(&mut args);
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
            .act(&self.address, 1, 3, 8, args.freeze())
            .await?;
        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SubObjectInfoReply {
    module_id: u16,
    node_id: u16,
    object_id: u16,
}

#[derive(Clone, Debug)]
pub struct ZSeekObstaclePositionReply {
    z_position: Vec<i16>,
    obstacle_detected: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct StructInfoReply {
    struct_names: Vec<String>,
    number_structure_elements: Vec<u32>,
    structure_element_types: Vec<u8>,
    structure_element_descriptions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct IsCoreGripperToolHeldReply {
    gripped: bool,
    tip_type: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct GetRemainingChannelsReply {
    channels: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct CreateLimitCurveReply {
    index: u32,
}

#[derive(Clone, Debug)]
pub struct ZSeekLldPositionReply {
    z_position: Vec<i16>,
    lld_detected: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct GetLimitCurveNamesReply {
    names: String,
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
pub struct GetPositionReply {
    x_position: i32,
    y_position: Vec<i16>,
    z_position: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct PickupPlateSuctionReply {
    first_pressure_change: i16,
    second_pressure_change: i16,
}

#[derive(Clone, Debug)]
pub struct RetrieveTadmDataReply {
    offsets: Vec<u16>,
    tadm_data: Vec<i16>,
}

#[derive(Clone, Debug)]
pub struct EnumInfoReply {
    enumeration_names: Vec<String>,
    number_enumeration_values: Vec<u32>,
    enumeration_values: Vec<i16>,
    enumeration_value_descriptions: Vec<String>,
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
pub struct GetChannelConfigurationReply {
    enabled: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct ObjectInfoReply {
    name: String,
    version: String,
    methods: u32,
    subobjects: u16,
}

#[derive(Clone, Debug)]
pub struct DropPlateSuctionReply {
    first_pressure_change: i16,
    second_pressure_change: i16,
}

#[derive(Clone, Debug)]
pub struct InterfaceDescriptorsReply {
    interface_ids: Vec<u8>,
    interface_descriptors: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CheckPlateSuctionReply {
    first_channel_has_plate: i16,
    second_channel_has_plate: i16,
}
