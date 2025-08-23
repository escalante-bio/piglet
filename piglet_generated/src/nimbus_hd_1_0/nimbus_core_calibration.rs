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

pub struct NimbusCoreCalibration {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreCalibration {
    // version 1.0

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 258,
            },
            robot: robot.clone(),
        }
    }

    pub async fn calibration_start(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 1, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationStart()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_save(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationSave()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_cancel(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationCancel()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pre_calibrate_z(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.PreCalibrateZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pre_calibrate_x(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
        x_cal_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.PreCalibrateX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pre_calibrate_y(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        y_distance: i32,
        y_cal_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        y_distance.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.PreCalibrateY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_x_resolution_start(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
    ) -> Result<CalibrateXResolutionStartReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateXResolutionStart(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_measured = i32::deserialize(&mut stream)?;
        Ok(CalibrateXResolutionStartReply { x_measured })
    }

    pub async fn calibrate_x_resolution_finish(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
    ) -> Result<CalibrateXResolutionFinishReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateXResolutionFinish(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_measured = i32::deserialize(&mut stream)?;
        let x_resolution = i32::deserialize(&mut stream)?;
        Ok(CalibrateXResolutionFinishReply {
            x_measured,
            x_resolution,
        })
    }

    pub async fn calibrate_channel_xy(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
        y_distance: i32,
        x_cal_position: i32,
        y_cal_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        y_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 9, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateChannelXY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_channel_z(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 10, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateChannelZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_check_x(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
        x_cal_position: i32,
    ) -> Result<CalibrationCheckXReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 11, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationCheckX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationCheckXReply { x_offset })
    }

    pub async fn calibration_check_y(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        y_distance: i32,
        y_cal_position: i32,
    ) -> Result<CalibrationCheckYReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        y_distance.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 12, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationCheckY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationCheckYReply { y_offset })
    }

    pub async fn calibration_check_z(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<CalibrationCheckZReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 13, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationCheckZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationCheckZReply { z_offset })
    }

    pub async fn calibration_check_dispenser(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        tip_volume: u32,
        tip_collet_check: i16,
    ) -> Result</* steps= */ i32, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_volume.serialize(&mut args);
        tip_collet_check.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 14, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  tip_volume: {:?}", tip_volume),
                    format!("  tip_collet_check: {:?}", tip_collet_check),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationCheckDispenser(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let steps = i32::deserialize(&mut stream)?;
        Ok(steps)
    }

    pub async fn calibrate_squeeze(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_start_position: i32,
        z_stop_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 15, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateSqueeze(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_tip_height(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_start_position: i32,
        z_stop_position: i32,
        z_final: i32,
        tip_volume: u32,
        tip_collet_check: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_start_position.serialize(&mut args);
        z_stop_position.serialize(&mut args);
        z_final.serialize(&mut args);
        tip_volume.serialize(&mut args);
        tip_collet_check.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 16, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_start_position: {:?}", z_start_position),
                    format!("  z_stop_position: {:?}", z_stop_position),
                    format!("  z_final: {:?}", z_final),
                    format!("  tip_volume: {:?}", tip_volume),
                    format!("  tip_collet_check: {:?}", tip_collet_check),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateTipHeight(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_touchoff(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 17, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateTouchoff(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_start(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 18, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrationStart()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_cancel(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 19, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrationCancel()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_save(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 20, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrationSave()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_pre_calibrate_z(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<GripPreCalibrateZReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 21, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripPreCalibrateZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_offset = i32::deserialize(&mut stream)?;
        Ok(GripPreCalibrateZReply { z_offset })
    }

    pub async fn grip_calibrate_xy(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        x_distance: i32,
        y_distance: i32,
        x_cal_position: i32,
        y_cal_position: i32,
    ) -> Result<GripCalibrateXYReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        x_distance.serialize(&mut args);
        y_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 22, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrateXY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        let y_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrateXYReply { x_offset, y_offset })
    }

    pub async fn grip_calibration_tool_setup(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 23, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrationToolSetup()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_tool_pickup(&self, tool_width: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tool_width.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 24, args.freeze()).await,
            || {
                let parameters = vec![format!("  tool_width: {:?}", tool_width)];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationToolPickup(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_auto_tool_pickup(
        &self,

        x_position: i32,
        y_position: i32,
        z_position: i32,
        grip_open: i32,
        tool_width: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        grip_open.serialize(&mut args);
        tool_width.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 25, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  grip_open: {:?}", grip_open),
                    format!("  tool_width: {:?}", tool_width),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationAutoToolPickup(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_tool_drop(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 26, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrationToolDrop()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibrate_wrist(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        y_distance: i32,
    ) -> Result<GripCalibrateWristReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        y_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 27, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  y_distance: {:?}", y_distance),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrateWrist(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let wrist_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrateWristReply { wrist_offset })
    }

    pub async fn grip_calibrate_z(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<GripCalibrateZReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 28, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrateZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrateZReply { z_offset })
    }

    pub async fn grip_calibration_grip(&self, tool_width: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tool_width.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 29, args.freeze()).await,
            || {
                let parameters = vec![format!("  tool_width: {:?}", tool_width)];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationGrip(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibration_check_x(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        x_distance: i32,
        x_cal_position: i32,
    ) -> Result<GripCalibrationCheckXReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        x_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 30, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationCheckX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrationCheckXReply { x_offset })
    }

    pub async fn grip_calibration_check_y(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        y_distance: i32,
        y_cal_position: i32,
    ) -> Result<GripCalibrationCheckYReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        y_distance.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 31, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationCheckY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrationCheckYReply { y_offset })
    }

    pub async fn grip_calibration_check_z(
        &self,

        lld_channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        tool_height: i32,
        z_distance: i32,
        z_cal_position: i32,
    ) -> Result<GripCalibrationCheckZReply, Error> {
        let mut args = BytesMut::new();
        lld_channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        tool_height.serialize(&mut args);
        z_distance.serialize(&mut args);
        z_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 32, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lld_channel: {:?}", lld_channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  tool_height: {:?}", tool_height),
                    format!("  z_distance: {:?}", z_distance),
                    format!("  z_cal_position: {:?}", z_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.GripCalibrationCheckZ(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_offset = i32::deserialize(&mut stream)?;
        Ok(GripCalibrationCheckZReply { z_offset })
    }

    pub async fn get_pressure(&self) -> Result</* pressures= */ Vec<i16>, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 33, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GetPressure()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let pressures = Vec::<i16>::deserialize(&mut stream)?;
        Ok(pressures)
    }

    pub async fn get_potentiometer_settings(&self) -> Result<GetPotentiometerSettingsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 34, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GetPotentiometerSettings()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let gain = Vec::<i16>::deserialize(&mut stream)?;
        let offset = Vec::<i16>::deserialize(&mut stream)?;
        Ok(GetPotentiometerSettingsReply { gain, offset })
    }

    pub async fn set_pressure_measurement_mode(
        &self,

        channel: u16,
        mode: i16,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        mode.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 35, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  mode: {:?}", mode),
                ];
                format!(
                    "in call to NimbusCoreCalibration.SetPressureMeasurementMode(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_pressure_sensor_gain(&self, channel: u16, gain: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        gain.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 36, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  gain: {:?}", gain),
                ];
                format!(
                    "in call to NimbusCoreCalibration.SetPressureSensorGain(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_pressure_sensor_offset(&self, channel: u16, offset: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 37, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  offset: {:?}", offset),
                ];
                format!(
                    "in call to NimbusCoreCalibration.SetPressureSensorOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_pressure_sensor_shift(&self, channel: u16, shift: i16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        shift.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 38, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  shift: {:?}", shift),
                ];
                format!(
                    "in call to NimbusCoreCalibration.SetPressureSensorShift(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn store_pressure_sensor_gain(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 39, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCoreCalibration.StorePressureSensorGain(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn store_pressure_sensor_offset(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 40, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCoreCalibration.StorePressureSensorOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_tadm_offset(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 41, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateTadmOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 42, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn leak_check(
        &self,

        tips_used: impl AsRef<[u16]>,
        x_position: impl AsRef<[i32]>,
        y_position: impl AsRef<[i32]>,
        traverse_height: i32,
        z_start_position: impl AsRef<[i32]>,
        z_stop_position: impl AsRef<[i32]>,
        z_final: impl AsRef<[i32]>,
        tip_type: impl AsRef<[u16]>,
        time: impl AsRef<[u32]>,
        test_type: impl AsRef<[bool]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        x_position.as_ref().serialize(&mut args);
        y_position.as_ref().serialize(&mut args);
        traverse_height.serialize(&mut args);
        z_start_position.as_ref().serialize(&mut args);
        z_stop_position.as_ref().serialize(&mut args);
        z_final.as_ref().serialize(&mut args);
        tip_type.as_ref().serialize(&mut args);
        time.as_ref().serialize(&mut args);
        test_type.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 43, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  tips_used: {:?}", tips_used.as_ref()),
                    format!("  x_position: {:?}", x_position.as_ref()),
                    format!("  y_position: {:?}", y_position.as_ref()),
                    format!("  traverse_height: {:?}", traverse_height),
                    format!("  z_start_position: {:?}", z_start_position.as_ref()),
                    format!("  z_stop_position: {:?}", z_stop_position.as_ref()),
                    format!("  z_final: {:?}", z_final.as_ref()),
                    format!("  tip_type: {:?}", tip_type.as_ref()),
                    format!("  time: {:?}", time.as_ref()),
                    format!("  test_type: {:?}", test_type.as_ref()),
                ];
                format!(
                    "in call to NimbusCoreCalibration.LeakCheck(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_lld(&self, channel: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 44, args.freeze()).await,
            || {
                let parameters = vec![format!("  channel: {:?}", channel)];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateLld(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_store(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 45, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationStore()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_squeeze_check_torque(
        &self,

        tips_used: impl AsRef<[u16]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 46, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used.as_ref())];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSqueezeCheckTorque(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibrate_squeeze_position(
        &self,

        tips_used: impl AsRef<[u16]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 47, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used.as_ref())];
                format!(
                    "in call to NimbusCoreCalibration.CalibrateSqueezePosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_reset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 48, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationReset()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_values(&self) -> Result<CalibrationValuesReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 49, args.freeze()).await,
            || "in call to NimbusCoreCalibration.CalibrationValues()".to_string(),
        )?;

        if count != 4 {
            return Err(ConnectionError(anyhow!("Expected 4 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        let x_resolution = i32::deserialize(&mut stream)?;
        let y_offset = Vec::<i32>::deserialize(&mut stream)?;
        let z_offset = Vec::<i32>::deserialize(&mut stream)?;
        Ok(CalibrationValuesReply {
            x_offset,
            x_resolution,
            y_offset,
            z_offset,
        })
    }

    pub async fn calibration_set_x_home_offset(&self, x_offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 50, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_offset: {:?}", x_offset)];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSetXHomeOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_set_x_resolution(&self, x_resolution: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_resolution.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 51, args.freeze()).await,
            || {
                let parameters = vec![format!("  x_resolution: {:?}", x_resolution)];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSetXResolution(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_set_y_home_offsets(
        &self,

        y_home_offset: impl AsRef<[i32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        y_home_offset.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 52, args.freeze()).await,
            || {
                let parameters = vec![format!("  y_home_offset: {:?}", y_home_offset.as_ref())];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSetYHomeOffsets(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_set_z_home_offsets(
        &self,

        z_home_offset: impl AsRef<[i32]>,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        z_home_offset.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 53, args.freeze()).await,
            || {
                let parameters = vec![format!("  z_home_offset: {:?}", z_home_offset.as_ref())];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSetZHomeOffsets(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn calibration_seek_x(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
    ) -> Result<CalibrationSeekXReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 54, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSeekX(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationSeekXReply { x_offset })
    }

    pub async fn calibration_seek_y(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        y_distance: i32,
    ) -> Result<CalibrationSeekYReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        y_distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 55, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  y_distance: {:?}", y_distance),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationSeekY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let y_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationSeekYReply { y_offset })
    }

    pub async fn calibration_barcode_laser(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 56, args.freeze()).await,
            || {
                let parameters = vec![format!("  enable: {:?}", enable)];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationBarcodeLaser(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_servo_off(&self, tips_used: impl AsRef<[u16]>) -> Result<(), Error> {
        let mut args = BytesMut::new();
        tips_used.as_ref().serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 57, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips_used: {:?}", tips_used.as_ref())];
                format!(
                    "in call to NimbusCoreCalibration.ZServoOff(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn grip_calibrate_grip_travel_extent(
        &self,
    ) -> Result<GripCalibrateGripTravelExtentReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 58, args.freeze()).await,
            || "in call to NimbusCoreCalibration.GripCalibrateGripTravelExtent()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let lower_limit = i32::deserialize(&mut stream)?;
        let upper_limit = i32::deserialize(&mut stream)?;
        Ok(GripCalibrateGripTravelExtentReply {
            lower_limit,
            upper_limit,
        })
    }

    pub async fn calibration_check_xy(
        &self,

        channel: u16,
        x_position: i32,
        y_position: i32,
        z_position: i32,
        x_distance: i32,
        y_distance: i32,
        x_cal_position: i32,
        y_cal_position: i32,
    ) -> Result<CalibrationCheckXYReply, Error> {
        let mut args = BytesMut::new();
        channel.serialize(&mut args);
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        x_distance.serialize(&mut args);
        y_distance.serialize(&mut args);
        x_cal_position.serialize(&mut args);
        y_cal_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 59, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  channel: {:?}", channel),
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  x_distance: {:?}", x_distance),
                    format!("  y_distance: {:?}", y_distance),
                    format!("  x_cal_position: {:?}", x_cal_position),
                    format!("  y_cal_position: {:?}", y_cal_position),
                ];
                format!(
                    "in call to NimbusCoreCalibration.CalibrationCheckXY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let x_offset = i32::deserialize(&mut stream)?;
        let y_offset = i32::deserialize(&mut stream)?;
        Ok(CalibrationCheckXYReply { x_offset, y_offset })
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreCalibration.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreCalibration.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreCalibration.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreCalibration.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreCalibration.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreCalibration.StructInfo(\n{}\n)",
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
pub struct CalibrateXResolutionStartReply {
    x_measured: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrateXResolutionFinishReply {
    x_measured: i32,
    x_resolution: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationCheckXReply {
    x_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationCheckYReply {
    y_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationCheckZReply {
    z_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripPreCalibrateZReply {
    z_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrateXYReply {
    x_offset: i32,
    y_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrateWristReply {
    wrist_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrateZReply {
    z_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrationCheckXReply {
    x_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrationCheckYReply {
    y_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrationCheckZReply {
    z_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetPotentiometerSettingsReply {
    gain: Vec<i16>,
    offset: Vec<i16>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationValuesReply {
    x_offset: i32,
    x_resolution: i32,
    y_offset: Vec<i32>,
    z_offset: Vec<i32>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationSeekXReply {
    x_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationSeekYReply {
    y_offset: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GripCalibrateGripTravelExtentReply {
    lower_limit: i32,
    upper_limit: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CalibrationCheckXYReply {
    x_offset: i32,
    y_offset: i32,
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
