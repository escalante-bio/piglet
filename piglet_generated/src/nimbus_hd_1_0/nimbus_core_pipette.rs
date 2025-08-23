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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 1, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                ];
                format!(
                    "in call to NimbusCorePipette.Initialize(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                ];
                format!(
                    "in call to NimbusCorePipette.InitializeSmart(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tip_type: {:?}", tip_type),
                    format!("  volume: {:?}", volume),
                    format!("  length: {:?}", length),
                    format!("  collar_type: {:?}", collar_type),
                    format!("  filtered: {:?}", filtered),
                    format!("  needle: {:?}", needle),
                ];
                format!(
                    "in call to NimbusCorePipette.TipAndNeedleDefinition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  tip_type: {:?}", tip_type),
                ];
                format!(
                    "in call to NimbusCorePipette.PickupTips(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  default_waste: {:?}", default_waste),
                ];
                format!(
                    "in call to NimbusCorePipette.DropTips(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  aspirate_type: {:?}", aspirate_type),
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  liquid_seek_height: {:?}", liquid_seek_height),
                    format!("  liquid_surface_height: {:?}", liquid_surface_height),
                    format!("  submerge_depth: {:?}", submerge_depth),
                    format!("  follow_depth: {:?}", follow_depth),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  clot_check_height: {:?}", clot_check_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  liquid_exit_speed: {:?}", liquid_exit_speed),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  prewet_volume: {:?}", prewet_volume),
                    format!("  aspirate_volume: {:?}", aspirate_volume),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                    format!("  aspirate_speed: {:?}", aspirate_speed),
                    format!("  settling_time: {:?}", settling_time),
                    format!("  mix_volume: {:?}", mix_volume),
                    format!("  mix_cycles: {:?}", mix_cycles),
                    format!("  mix_position: {:?}", mix_position),
                    format!("  mix_follow_distance: {:?}", mix_follow_distance),
                    format!("  mix_speed: {:?}", mix_speed),
                    format!("  tube_section_height: {:?}", tube_section_height),
                    format!("  tube_section_ratio: {:?}", tube_section_ratio),
                    format!("  lld_mode: {:?}", lld_mode),
                    format!(
                        "  capacitive_lld_sensitivity: {:?}",
                        capacitive_lld_sensitivity
                    ),
                    format!("  pressure_lld_sensitivity: {:?}", pressure_lld_sensitivity),
                    format!("  lld_height_difference: {:?}", lld_height_difference),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                ];
                format!(
                    "in call to NimbusCorePipette.Aspirate_1(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  aspirate_type: {:?}", aspirate_type),
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  liquid_seek_height: {:?}", liquid_seek_height),
                    format!("  liquid_surface_height: {:?}", liquid_surface_height),
                    format!("  submerge_depth: {:?}", submerge_depth),
                    format!("  follow_depth: {:?}", follow_depth),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  clot_check_height: {:?}", clot_check_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  liquid_exit_speed: {:?}", liquid_exit_speed),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  prewet_volume: {:?}", prewet_volume),
                    format!("  aspirate_volume: {:?}", aspirate_volume),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                    format!("  aspirate_speed: {:?}", aspirate_speed),
                    format!("  settling_time: {:?}", settling_time),
                    format!("  mix_volume: {:?}", mix_volume),
                    format!("  mix_cycles: {:?}", mix_cycles),
                    format!("  mix_position: {:?}", mix_position),
                    format!("  mix_follow_distance: {:?}", mix_follow_distance),
                    format!("  mix_speed: {:?}", mix_speed),
                    format!("  tube_section_height: {:?}", tube_section_height),
                    format!("  tube_section_ratio: {:?}", tube_section_ratio),
                    format!("  lld_mode: {:?}", lld_mode),
                    format!(
                        "  capacitive_lld_sensitivity: {:?}",
                        capacitive_lld_sensitivity
                    ),
                    format!("  pressure_lld_sensitivity: {:?}", pressure_lld_sensitivity),
                    format!("  lld_height_difference: {:?}", lld_height_difference),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                ];
                format!(
                    "in call to NimbusCorePipette.Aspirate_2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  dispense_type: {:?}", dispense_type),
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  liquid_seek_height: {:?}", liquid_seek_height),
                    format!("  dispense_height: {:?}", dispense_height),
                    format!("  submerge_depth: {:?}", submerge_depth),
                    format!("  follow_depth: {:?}", follow_depth),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  liquid_exit_speed: {:?}", liquid_exit_speed),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                    format!("  dispense_volume: {:?}", dispense_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  settling_time: {:?}", settling_time),
                    format!("  mix_volume: {:?}", mix_volume),
                    format!("  mix_cycles: {:?}", mix_cycles),
                    format!("  mix_position: {:?}", mix_position),
                    format!("  mix_follow_distance: {:?}", mix_follow_distance),
                    format!("  mix_speed: {:?}", mix_speed),
                    format!("  touch_off_distance: {:?}", touch_off_distance),
                    format!("  dispense_offset: {:?}", dispense_offset),
                    format!("  tube_section_height: {:?}", tube_section_height),
                    format!("  tube_section_ratio: {:?}", tube_section_ratio),
                    format!("  lld_mode: {:?}", lld_mode),
                    format!(
                        "  capacitive_lld_sensitivity: {:?}",
                        capacitive_lld_sensitivity
                    ),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                ];
                format!(
                    "in call to NimbusCorePipette.Dispense(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  aspirate_type: {:?}", aspirate_type),
                    format!("  dispense_type: {:?}", dispense_type),
                    format!("  tips_used: {:?}", tips_used),
                    format!(
                        "  aspirate_dispense_pattern: {:?}",
                        aspirate_dispense_pattern
                    ),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  liquid_seek_height: {:?}", liquid_seek_height),
                    format!("  liquid_surface_height: {:?}", liquid_surface_height),
                    format!("  submerge_depth: {:?}", submerge_depth),
                    format!("  follow_depth: {:?}", follow_depth),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  clot_check_height: {:?}", clot_check_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  liquid_exit_speed: {:?}", liquid_exit_speed),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  prewet_volume: {:?}", prewet_volume),
                    format!("  aspirate_volume: {:?}", aspirate_volume),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                    format!("  dispense_volume: {:?}", dispense_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  aspirate_speed: {:?}", aspirate_speed),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  settling_time: {:?}", settling_time),
                    format!("  mix_volume: {:?}", mix_volume),
                    format!("  mix_cycles: {:?}", mix_cycles),
                    format!("  mix_position: {:?}", mix_position),
                    format!("  mix_follow_distance: {:?}", mix_follow_distance),
                    format!("  mix_speed: {:?}", mix_speed),
                    format!("  dispense_offset: {:?}", dispense_offset),
                    format!("  tube_section_height: {:?}", tube_section_height),
                    format!("  tube_section_ratio: {:?}", tube_section_ratio),
                    format!("  lld_mode: {:?}", lld_mode),
                    format!(
                        "  capacitive_lld_sensitivity: {:?}",
                        capacitive_lld_sensitivity
                    ),
                    format!("  pressure_lld_sensitivity: {:?}", pressure_lld_sensitivity),
                    format!("  lld_height_difference: {:?}", lld_height_difference),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                ];
                format!(
                    "in call to NimbusCorePipette.AspirateAndDispense(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 9, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position_1_st_channel: {:?}", y_position_1_st_channel),
                    format!("  y_position_2_nd_channel: {:?}", y_position_2_nd_channel),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  tip_type: {:?}", tip_type),
                    format!("  first_channel_number: {:?}", first_channel_number),
                    format!("  second_channel_number: {:?}", second_channel_number),
                    format!("  tool_width: {:?}", tool_width),
                ];
                format!(
                    "in call to NimbusCorePipette.PickupGripperTool(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 10, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position_1_st_channel: {:?}", y_position_1_st_channel),
                    format!("  y_position_2_nd_channel: {:?}", y_position_2_nd_channel),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  first_channel_number: {:?}", first_channel_number),
                    format!("  second_channel_number: {:?}", second_channel_number),
                ];
                format!(
                    "in call to NimbusCorePipette.DropGripperTool(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 11, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  y_plate_width: {:?}", y_plate_width),
                    format!("  y_open_position: {:?}", y_open_position),
                    format!("  y_grip_speed: {:?}", y_grip_speed),
                    format!("  y_grip_strength: {:?}", y_grip_strength),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_grip_height: {:?}", z_grip_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.PickupPlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 12, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  y_open_position: {:?}", y_open_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_drop_height: {:?}", z_drop_height),
                    format!("  z_press_distance: {:?}", z_press_distance),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.DropPlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 13, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.MovePlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn release_plate(
        &self,

        first_channel_number: u16,
        second_channel_number: u16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        first_channel_number.serialize(&mut args);
        second_channel_number.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 14, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  first_channel_number: {:?}", first_channel_number),
                    format!("  second_channel_number: {:?}", second_channel_number),
                ];
                format!(
                    "in call to NimbusCorePipette.ReleasePlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_initialized(&self) -> Result</* initialized= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 15, args.freeze()).await,
            || "in call to NimbusCorePipette.IsInitialized()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(initialized)
    }

    pub async fn is_tip_present(&self) -> Result</* tip_present= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 16, args.freeze()).await,
            || "in call to NimbusCorePipette.IsTipPresent()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_present = Vec::<i16>::deserialize(&mut stream)?;
        Ok(tip_present)
    }

    pub async fn is_core_gripper_tool_held(&self) -> Result<IsCoreGripperToolHeldReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 17, args.freeze()).await,
            || "in call to NimbusCorePipette.IsCoreGripperToolHeld()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let gripped = bool::deserialize(&mut stream)?;
        let tip_type = Vec::<u16>::deserialize(&mut stream)?;
        Ok(IsCoreGripperToolHeldReply { gripped, tip_type })
    }

    pub async fn is_core_gripper_plate_gripped(&self) -> Result</* gripped= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 18, args.freeze()).await,
            || "in call to NimbusCorePipette.IsCoreGripperPlateGripped()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let gripped = bool::deserialize(&mut stream)?;
        Ok(gripped)
    }

    pub async fn get_tip_and_needle_types(&self) -> Result</* tip_type= */ Vec<u16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 19, args.freeze()).await,
            || "in call to NimbusCorePipette.GetTipAndNeedleTypes()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let tip_type = Vec::<u16>::deserialize(&mut stream)?;
        Ok(tip_type)
    }

    pub async fn get_position(&self) -> Result<GetPositionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 20, args.freeze()).await,
            || "in call to NimbusCorePipette.GetPosition()".to_string(),
        )?;

        if count != 3 {
            return Err(ConnectionError(anyhow!("Expected 3 values, not {}", count)));
        }
        let x_position = i32::deserialize(&mut stream)?;
        let y_position = Vec::<i32>::deserialize(&mut stream)?;
        let z_position = Vec::<i32>::deserialize(&mut stream)?;
        Ok(GetPositionReply {
            x_position,
            y_position,
            z_position,
        })
    }

    pub async fn park(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 21, args.freeze()).await,
            || "in call to NimbusCorePipette.Park()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 22, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_position: {:?}", z_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveOver(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 23, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveToPosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 24, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveToPositionViaLane(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 25, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveAbsoluteXY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_x(&self, x_position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 26, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_position: {:?}", x_position)];
                format!(
                    "in call to NimbusCorePipette.MoveAbsoluteX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_x(&self, x_distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 27, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_distance: {:?}", x_distance)];
                format!(
                    "in call to NimbusCorePipette.MoveRelativeX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 28, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  y_position: {:?}", y_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveAbsoluteY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 29, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  y_distance: {:?}", y_distance),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveRelativeY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 30, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  z_position: {:?}", z_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveAbsoluteZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 31, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  z_distance: {:?}", z_distance),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveRelativeZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn preinitialize_smart(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 32, args.freeze()).await,
            || "in call to NimbusCorePipette.PreinitializeSmart()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn erase_limit_curves(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 33, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCorePipette.EraseLimitCurves(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 34, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  name: {:?}", name),
                    format!("  lower_limit_x: {:?}", lower_limit_x),
                    format!("  lower_limit_y: {:?}", lower_limit_y),
                    format!("  upper_limit_x: {:?}", upper_limit_x),
                    format!("  upper_limit_y: {:?}", upper_limit_y),
                ];
                format!(
                    "in call to NimbusCorePipette.CreateLimitCurve(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let index = u32::deserialize(&mut stream)?;
        Ok(CreateLimitCurveReply { index })
    }

    pub async fn reset_tadm_fifo(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 35, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.ResetTadmFifo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 36, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.RetrieveTadmData(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 37, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCorePipette.GetLimitCurveNames(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 38, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  name: {:?}", name),
                ];
                format!(
                    "in call to NimbusCorePipette.GetLimitCurveInfo(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 39, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  enable: {:?}", enable),
                ];
                format!(
                    "in call to NimbusCorePipette.SetTADMEnable(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_tadm_enable(&self) -> Result<GetTADMEnableReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 40, args.freeze()).await,
            || "in call to NimbusCorePipette.GetTADMEnable()".to_string(),
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 41, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  pressure_threshold: {:?}", pressure_threshold),
                    format!(
                        "  minimum_pressure_difference: {:?}",
                        minimum_pressure_difference
                    ),
                    format!(
                        "  maximum_pressure_difference: {:?}",
                        maximum_pressure_difference
                    ),
                ];
                format!(
                    "in call to NimbusCorePipette.EnableMAD(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn disable_mad(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 42, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.DisableMAD(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn enable_adc(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 43, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.EnableADC(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn disable_adc(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 44, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.DisableADC(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 45, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  z_position: {:?}", z_position),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveTraverseZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 46, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  y_suction_offset: {:?}", y_suction_offset),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  suction_volume: {:?}", suction_volume),
                    format!("  pressure_differential: {:?}", pressure_differential),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_grip_height: {:?}", z_grip_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.PickupPlateSuction(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 47, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  pressure_differential: {:?}", pressure_differential),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_drop_height: {:?}", z_drop_height),
                    format!("  z_press_distance: {:?}", z_press_distance),
                    format!("  z_lift_distance: {:?}", z_lift_distance),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.DropPlateSuction(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 48, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  pressure_differential: {:?}", pressure_differential),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.MovePlateSuction(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 49, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  obstacle_seek_height: {:?}", obstacle_seek_height),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  seek_speed: {:?}", seek_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.ZSeekObstacle(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let obstacle_detected = Vec::<i16>::deserialize(&mut stream)?;
        Ok(obstacle_detected)
    }

    pub async fn get_remaining_channels(&self) -> Result<GetRemainingChannelsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 50, args.freeze()).await,
            || "in call to NimbusCorePipette.GetRemainingChannels()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let channels = Vec::<u16>::deserialize(&mut stream)?;
        Ok(GetRemainingChannelsReply { channels })
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 51, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  z_speed: {:?}", z_speed),
                    format!("  time: {:?}", time),
                    format!("  time_limit: {:?}", time_limit),
                ];
                format!(
                    "in call to NimbusCorePipette.SetPerformanceParameters(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn reset_performance_parameters(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 52, args.freeze()).await,
            || "in call to NimbusCorePipette.ResetPerformanceParameters()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_x(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 53, args.freeze()).await,
            || "in call to NimbusCorePipette.InitializeX()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_y(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 54, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.InitializeY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_z(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 55, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.InitializeZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_squeeze(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 56, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.InitializeSqueeze(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_dispenser(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 57, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.InitializeDispenser(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn squeeze_on(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 58, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.SqueezeOn(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn squeeze_off(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 59, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.SqueezeOff(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn empty_dispenser(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 60, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.EmptyDispenser(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 61, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  volumes: {:?}", volumes),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveRelativeDispenser(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 62, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  x_well_distance: {:?}", x_well_distance),
                    format!("  x_well_width: {:?}", x_well_width),
                    format!("  x_number_of_wells: {:?}", x_number_of_wells),
                    format!("  x_velocity_scale: {:?}", x_velocity_scale),
                    format!("  y_position: {:?}", y_position),
                    format!("  y_well_distance: {:?}", y_well_distance),
                    format!("  volume: {:?}", volume),
                    format!("  tip_air_volume: {:?}", tip_air_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  dispense_height: {:?}", dispense_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  dispense_map: {:?}", dispense_map),
                ];
                format!(
                    "in call to NimbusCorePipette.MultipleDispense(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 63, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  x_well_distance: {:?}", x_well_distance),
                    format!("  x_well_width: {:?}", x_well_width),
                    format!("  x_number_of_wells: {:?}", x_number_of_wells),
                    format!("  x_velocity_scale: {:?}", x_velocity_scale),
                    format!("  y_position: {:?}", y_position),
                    format!("  y_well_distance: {:?}", y_well_distance),
                    format!("  volume: {:?}", volume),
                    format!("  tip_air_volume: {:?}", tip_air_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  dispense_height: {:?}", dispense_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  dispense_map: {:?}", dispense_map),
                ];
                format!(
                    "in call to NimbusCorePipette.OnTheFlyDispense(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_current_dispenser_volume(&self) -> Result</* volume= */ Vec<u32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 64, args.freeze()).await,
            || "in call to NimbusCorePipette.GetCurrentDispenserVolume()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let volume = Vec::<u32>::deserialize(&mut stream)?;
        Ok(volume)
    }

    pub async fn get_liquid_height(&self) -> Result</* liquid_height= */ Vec<i32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 65, args.freeze()).await,
            || "in call to NimbusCorePipette.GetLiquidHeight()".to_string(),
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 66, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  indexes: {:?}", indexes),
                ];
                format!(
                    "in call to NimbusCorePipette.GetChannelConfiguration(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 67, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  indexes: {:?}", indexes),
                    format!("  enables: {:?}", enables),
                ];
                format!(
                    "in call to NimbusCorePipette.SetChannelConfiguration(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 68, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  obstacle_seek_height: {:?}", obstacle_seek_height),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  seek_speed: {:?}", seek_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.ZSeekObstaclePosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 69, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  seek_height: {:?}", seek_height),
                    format!("  z_min_position: {:?}", z_min_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  seek_speed: {:?}", seek_speed),
                    format!(
                        "  capacitive_lld_sensitivity: {:?}",
                        capacitive_lld_sensitivity
                    ),
                ];
                format!(
                    "in call to NimbusCorePipette.ZSeekLldPosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 70, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                ];
                format!(
                    "in call to NimbusCorePipette.DispenseInitializeToWaste(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 71, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  x_well_distance: {:?}", x_well_distance),
                    format!("  x_well_width: {:?}", x_well_width),
                    format!("  x_number_of_wells: {:?}", x_number_of_wells),
                    format!("  x_velocity_scale: {:?}", x_velocity_scale),
                    format!("  y_position: {:?}", y_position),
                    format!("  y_well_distance: {:?}", y_well_distance),
                    format!("  volume: {:?}", volume),
                    format!("  tip_air_volume: {:?}", tip_air_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  dispense_height: {:?}", dispense_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  dispense_map: {:?}", dispense_map),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                ];
                format!(
                    "in call to NimbusCorePipette.MultipleDispenseTransportAir(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 72, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  x_well_distance: {:?}", x_well_distance),
                    format!("  x_well_width: {:?}", x_well_width),
                    format!("  x_number_of_wells: {:?}", x_number_of_wells),
                    format!("  x_velocity_scale: {:?}", x_velocity_scale),
                    format!("  y_position: {:?}", y_position),
                    format!("  y_well_distance: {:?}", y_well_distance),
                    format!("  volume: {:?}", volume),
                    format!("  tip_air_volume: {:?}", tip_air_volume),
                    format!("  stop_back_volume: {:?}", stop_back_volume),
                    format!("  dispense_speed: {:?}", dispense_speed),
                    format!("  cutoff_speed: {:?}", cutoff_speed),
                    format!("  tadm_enabled: {:?}", tadm_enabled),
                    format!("  limit_curve_index: {:?}", limit_curve_index),
                    format!("  recording_mode: {:?}", recording_mode),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  dispense_height: {:?}", dispense_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  dispense_map: {:?}", dispense_map),
                    format!("  transport_air_volume: {:?}", transport_air_volume),
                ];
                format!(
                    "in call to NimbusCorePipette.OnTheFlyDispenseTransportAir(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 73, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  seek_speeds: {:?}", seek_speeds),
                ];
                format!(
                    "in call to NimbusCorePipette.SetZLiquidSeekSpeed(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_z_liquid_seek_speed(&self) -> Result</* seek_speeds= */ Vec<u32>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 74, args.freeze()).await,
            || "in call to NimbusCorePipette.GetZLiquidSeekSpeed()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let seek_speeds = Vec::<u32>::deserialize(&mut stream)?;
        Ok(seek_speeds)
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 75, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  z_position: {:?}", z_position),
                    format!("  z_speed_switch_position: {:?}", z_speed_switch_position),
                    format!("  velocity: {:?}", velocity),
                    format!("  lower_section_velocity: {:?}", lower_section_velocity),
                ];
                format!(
                    "in call to NimbusCorePipette.MoveAbsoluteZ2Speed(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 76, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  y_suction_offset: {:?}", y_suction_offset),
                    format!("  blowout_volume: {:?}", blowout_volume),
                    format!("  suction_volume: {:?}", suction_volume),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_grip_height: {:?}", z_grip_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.PickupPlateSuction2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 77, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_drop_height: {:?}", z_drop_height),
                    format!("  z_press_distance: {:?}", z_press_distance),
                    format!("  z_lift_distance: {:?}", z_lift_distance),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.DropPlateSuction2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 78, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  x_acceleration: {:?}", x_acceleration),
                    format!("  y_plate_center_position: {:?}", y_plate_center_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_final: {:?}", z_final),
                    format!("  z_speed: {:?}", z_speed),
                ];
                format!(
                    "in call to NimbusCorePipette.MovePlateSuction2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 79, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  first_channel_number: {:?}", first_channel_number),
                    format!("  second_channel_number: {:?}", second_channel_number),
                    format!("  suction_volume: {:?}", suction_volume),
                ];
                format!(
                    "in call to NimbusCorePipette.CheckPlateSuction(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 80, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  roll_distance: {:?}", roll_distance),
                ];
                format!(
                    "in call to NimbusCorePipette.InitializeRoll(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 81, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  roll_distance: {:?}", roll_distance),
                ];
                format!(
                    "in call to NimbusCorePipette.InitializeSmartRoll(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

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
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 82, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  roll_distance: {:?}", roll_distance),
                ];
                format!(
                    "in call to NimbusCorePipette.DropTipsRoll(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_safe(&self, tips_used: Vec<u16>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 83, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used)];
                format!(
                    "in call to NimbusCorePipette.ZMoveSafe(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_x_interlock_active(&self) -> Result</* active= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 85, args.freeze()).await,
            || "in call to NimbusCorePipette.IsXInterlockActive()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let active = bool::deserialize(&mut stream)?;
        Ok(active)
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCorePipette.ObjectInfo()".to_string(),
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
                    "in call to NimbusCorePipette.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCorePipette.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCorePipette.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCorePipette.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCorePipette.StructInfo(\n{}\n)",
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
pub struct IsCoreGripperToolHeldReply {
    gripped: bool,
    tip_type: Vec<u16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetPositionReply {
    x_position: i32,
    y_position: Vec<i32>,
    z_position: Vec<i32>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CreateLimitCurveReply {
    index: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct RetrieveTadmDataReply {
    offsets: Vec<u16>,
    tadm_data: Vec<i16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetLimitCurveNamesReply {
    names: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetLimitCurveInfoReply {
    index: u32,
    lower_limits: u16,
    upper_limits: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetTADMEnableReply {
    enable: Vec<i16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct PickupPlateSuctionReply {
    first_pressure_change: i16,
    second_pressure_change: i16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct DropPlateSuctionReply {
    first_pressure_change: i16,
    second_pressure_change: i16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetRemainingChannelsReply {
    channels: Vec<u16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetChannelConfigurationReply {
    enabled: Vec<bool>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct ZSeekObstaclePositionReply {
    z_position: Vec<i32>,
    obstacle_detected: Vec<i16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct ZSeekLldPositionReply {
    z_position: Vec<i32>,
    lld_detected: Vec<i16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CheckPlateSuctionReply {
    first_channel_has_plate: i16,
    second_channel_has_plate: i16,
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
