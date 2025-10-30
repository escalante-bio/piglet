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

pub struct NimbusCoreXDrive {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreXDrive {
    // version 0.5

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 260,
            },
            robot: robot.clone(),
        }
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 1, args.freeze()).await,
            || "in call to NimbusCoreXDrive.Initialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_initialized(&self) -> Result</* initialized= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 2, args.freeze()).await,
            || "in call to NimbusCoreXDrive.IsInitialized()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let initialized = bool::deserialize(&mut stream)?;
        Ok(initialized)
    }

    pub async fn move_absolute_1(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreXDrive.MoveAbsolute_1(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_absolute_2(
        &self,

        position: i32,
        velocity: u32,
        acceleration: u32,
        restore_vel_acc: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        velocity.serialize(&mut args);
        acceleration.serialize(&mut args);
        restore_vel_acc.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  position: {:?}", position),
                    format!("  velocity: {:?}", velocity),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  restore_vel_acc: {:?}", restore_vel_acc),
                ];
                format!(
                    "in call to NimbusCoreXDrive.MoveAbsolute_2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_1(&self, distance: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![format!("  distance: {:?}", distance)];
                format!(
                    "in call to NimbusCoreXDrive.MoveRelative_1(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_relative_2(
        &self,

        distance: i32,
        velocity: u32,
        acceleration: u32,
        restore_vel_acc: bool,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        velocity.serialize(&mut args);
        acceleration.serialize(&mut args);
        restore_vel_acc.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  distance: {:?}", distance),
                    format!("  velocity: {:?}", velocity),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  restore_vel_acc: {:?}", restore_vel_acc),
                ];
                format!(
                    "in call to NimbusCoreXDrive.MoveRelative_2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn seek_to_position_flag_1(
        &self,

        distance: i32,
        travel_limits_enable: bool,
        trip_sense: i8,
        trip_flag: i8,
    ) -> Result</* trip_pos= */ i32, Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        travel_limits_enable.serialize(&mut args);
        trip_sense.serialize(&mut args);
        trip_flag.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  distance: {:?}", distance),
                    format!("  travel_limits_enable: {:?}", travel_limits_enable),
                    format!("  trip_sense: {:?}", trip_sense),
                    format!("  trip_flag: {:?}", trip_flag),
                ];
                format!(
                    "in call to NimbusCoreXDrive.SeekToPositionFlag_1(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let trip_pos = i32::deserialize(&mut stream)?;
        Ok(trip_pos)
    }

    pub async fn seek_to_position_flag_2(
        &self,

        distance: i32,
        travel_limits_enable: bool,
        trip_sense: i8,
        trip_flag: i8,
        velocity: u32,
        acceleration: u32,
        restore_vel_acc: bool,
    ) -> Result</* trip_pos= */ i32, Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        travel_limits_enable.serialize(&mut args);
        trip_sense.serialize(&mut args);
        trip_flag.serialize(&mut args);
        velocity.serialize(&mut args);
        acceleration.serialize(&mut args);
        restore_vel_acc.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  distance: {:?}", distance),
                    format!("  travel_limits_enable: {:?}", travel_limits_enable),
                    format!("  trip_sense: {:?}", trip_sense),
                    format!("  trip_flag: {:?}", trip_flag),
                    format!("  velocity: {:?}", velocity),
                    format!("  acceleration: {:?}", acceleration),
                    format!("  restore_vel_acc: {:?}", restore_vel_acc),
                ];
                format!(
                    "in call to NimbusCoreXDrive.SeekToPositionFlag_2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let trip_pos = i32::deserialize(&mut stream)?;
        Ok(trip_pos)
    }

    pub async fn get_latched_position_flag_position(&self) -> Result</* position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 9, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetLatchedPositionFlagPosition()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        Ok(position)
    }

    pub async fn get_commanded_position(&self) -> Result</* position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 10, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetCommandedPosition()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        Ok(position)
    }

    pub async fn get_encoder_position(&self) -> Result</* position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 11, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetEncoderPosition()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let position = i32::deserialize(&mut stream)?;
        Ok(position)
    }

    pub async fn set_velocity(&self, velocity: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        velocity.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 12, args.freeze()).await,
            || {
                let parameters = vec![format!("  velocity: {:?}", velocity)];
                format!(
                    "in call to NimbusCoreXDrive.SetVelocity(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_velocity(&self) -> Result</* velocity= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 13, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetVelocity()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let velocity = u32::deserialize(&mut stream)?;
        Ok(velocity)
    }

    pub async fn set_acceleration(&self, acceleration: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        acceleration.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 14, args.freeze()).await,
            || {
                let parameters = vec![format!("  acceleration: {:?}", acceleration)];
                format!(
                    "in call to NimbusCoreXDrive.SetAcceleration(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_acceleration(&self) -> Result</* acceleration= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 15, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetAcceleration()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let acceleration = u32::deserialize(&mut stream)?;
        Ok(acceleration)
    }

    pub async fn set_vel_acc(&self, velocity: u32, acceleration: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        velocity.serialize(&mut args);
        acceleration.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 16, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  velocity: {:?}", velocity),
                    format!("  acceleration: {:?}", acceleration),
                ];
                format!(
                    "in call to NimbusCoreXDrive.SetVelAcc(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_vel_acc(&self) -> Result<GetVelAccReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 17, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetVelAcc()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let velocity = u32::deserialize(&mut stream)?;
        let acceleration = u32::deserialize(&mut stream)?;
        Ok(GetVelAccReply {
            velocity,
            acceleration,
        })
    }

    pub async fn set_travel_limits(&self, lower_limit: i32, upper_limit: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        lower_limit.serialize(&mut args);
        upper_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 18, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  lower_limit: {:?}", lower_limit),
                    format!("  upper_limit: {:?}", upper_limit),
                ];
                format!(
                    "in call to NimbusCoreXDrive.SetTravelLimits(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_travel_limits(&self) -> Result<GetTravelLimitsReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 19, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetTravelLimits()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let lower_limit = i32::deserialize(&mut stream)?;
        let upper_limit = i32::deserialize(&mut stream)?;
        Ok(GetTravelLimitsReply {
            lower_limit,
            upper_limit,
        })
    }

    pub async fn set_travel_limits_offset(&self, offset: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        offset.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 20, args.freeze()).await,
            || {
                let parameters = vec![format!("  offset: {:?}", offset)];
                format!(
                    "in call to NimbusCoreXDrive.SetTravelLimitsOffset(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_travel_limits_offset(&self) -> Result</* offset= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 21, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetTravelLimitsOffset()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let offset = i32::deserialize(&mut stream)?;
        Ok(offset)
    }

    pub async fn travel_limits_enable(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 22, args.freeze()).await,
            || {
                let parameters = vec![format!("  enable: {:?}", enable)];
                format!(
                    "in call to NimbusCoreXDrive.TravelLimitsEnable(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_travel_limits_enabled(&self) -> Result</* enable= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 23, args.freeze()).await,
            || "in call to NimbusCoreXDrive.IsTravelLimitsEnabled()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = bool::deserialize(&mut stream)?;
        Ok(enable)
    }

    pub async fn set_home_position(&self, home_position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        home_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 24, args.freeze()).await,
            || {
                let parameters = vec![format!("  home_position: {:?}", home_position)];
                format!(
                    "in call to NimbusCoreXDrive.SetHomePosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_home_position(&self) -> Result</* home_position= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 25, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetHomePosition()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let home_position = i32::deserialize(&mut stream)?;
        Ok(home_position)
    }

    pub async fn get_sensor_states(&self) -> Result</* sensor_states= */ u8, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 26, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetSensorStates()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let sensor_states = u8::deserialize(&mut stream)?;
        Ok(sensor_states)
    }

    pub async fn get_status(&self) -> Result</* status= */ u32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 27, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetStatus()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let status = u32::deserialize(&mut stream)?;
        Ok(status)
    }

    pub async fn is_in_home_sensor(&self) -> Result</* in_home_sensor= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 28, args.freeze()).await,
            || "in call to NimbusCoreXDrive.IsInHomeSensor()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let in_home_sensor = bool::deserialize(&mut stream)?;
        Ok(in_home_sensor)
    }

    pub async fn servo_enable(&self, enable: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        enable.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 29, args.freeze()).await,
            || {
                let parameters = vec![format!("  enable: {:?}", enable)];
                format!(
                    "in call to NimbusCoreXDrive.ServoEnable(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_servo_enabled(&self) -> Result</* enable= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 30, args.freeze()).await,
            || "in call to NimbusCoreXDrive.IsServoEnabled()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let enable = bool::deserialize(&mut stream)?;
        Ok(enable)
    }

    pub async fn set_profile(&self, profile: i8) -> Result<(), Error> {
        let mut args = BytesMut::new();
        profile.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 31, args.freeze()).await,
            || {
                let parameters = vec![format!("  profile: {:?}", profile)];
                format!(
                    "in call to NimbusCoreXDrive.SetProfile(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_profile(&self) -> Result</* profile= */ i8, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 32, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetProfile()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let profile = i8::deserialize(&mut stream)?;
        Ok(profile)
    }

    pub async fn set_jerk_ratio(&self, jerk_ratio: u8) -> Result<(), Error> {
        let mut args = BytesMut::new();
        jerk_ratio.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 33, args.freeze()).await,
            || {
                let parameters = vec![format!("  jerk_ratio: {:?}", jerk_ratio)];
                format!(
                    "in call to NimbusCoreXDrive.SetJerkRatio(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_jerk_ratio(&self) -> Result</* jerk_ratio= */ u8, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 34, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetJerkRatio()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let jerk_ratio = u8::deserialize(&mut stream)?;
        Ok(jerk_ratio)
    }

    pub async fn reset(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 35, args.freeze()).await,
            || "in call to NimbusCoreXDrive.Reset()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn stop(&self, hard: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        hard.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 36, args.freeze()).await,
            || {
                let parameters = vec![format!("  hard: {:?}", hard)];
                format!(
                    "in call to NimbusCoreXDrive.Stop(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_j_positive_limit(&self, j_positive_limit: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        j_positive_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 37, args.freeze()).await,
            || {
                let parameters = vec![format!("  j_positive_limit: {:?}", j_positive_limit)];
                format!(
                    "in call to NimbusCoreXDrive.SetJPositiveLimit(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_j_positive_limit(&self) -> Result</* j_positive_limit= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 38, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetJPositiveLimit()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let j_positive_limit = i32::deserialize(&mut stream)?;
        Ok(j_positive_limit)
    }

    pub async fn get_max_j_positive_limit(&self) -> Result</* j_positive_limit_max= */ i32, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 39, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetMaxJPositiveLimit()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let j_positive_limit_max = i32::deserialize(&mut stream)?;
        Ok(j_positive_limit_max)
    }

    pub async fn compute_par_ms(&self, distance: i32) -> Result</* time= */ u32, Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 40, args.freeze()).await,
            || {
                let parameters = vec![format!("  distance: {:?}", distance)];
                format!(
                    "in call to NimbusCoreXDrive.ComputeParMs(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let time = u32::deserialize(&mut stream)?;
        Ok(time)
    }

    pub async fn compute_par_j_pos_limit(
        &self,

        distance: i32,
        time: u32,
    ) -> Result</* j_pos_limit= */ i32, Error> {
        let mut args = BytesMut::new();
        distance.serialize(&mut args);
        time.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 41, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  distance: {:?}", distance),
                    format!("  time: {:?}", time),
                ];
                format!(
                    "in call to NimbusCoreXDrive.ComputeParJPosLimit(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let j_pos_limit = i32::deserialize(&mut stream)?;
        Ok(j_pos_limit)
    }

    pub async fn configure_stop_soft_w_decel(&self, acc: u32, bit_mask: u32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        acc.serialize(&mut args);
        bit_mask.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 42, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  acc: {:?}", acc),
                    format!("  bit_mask: {:?}", bit_mask),
                ];
                format!(
                    "in call to NimbusCoreXDrive.ConfigureStopSoftWDecel(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn set_label(&self, label: &str) -> Result<(), Error> {
        let mut args = BytesMut::new();
        label.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 43, args.freeze()).await,
            || {
                let parameters = vec![format!("  label: {:?}", label)];
                format!(
                    "in call to NimbusCoreXDrive.SetLabel(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_label(&self) -> Result</* label= */ String, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 44, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetLabel()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let label = String::deserialize(&mut stream)?;
        Ok(label)
    }

    pub async fn get_settling_parameters(&self) -> Result<GetSettlingParametersReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 45, args.freeze()).await,
            || "in call to NimbusCoreXDrive.GetSettlingParameters()".to_string(),
        )?;

        if count != 2 {
            return Err(ConnectionError(anyhow!("Expected 2 values, not {}", count)));
        }
        let time = u16::deserialize(&mut stream)?;
        let time_limit = u16::deserialize(&mut stream)?;
        Ok(GetSettlingParametersReply { time, time_limit })
    }

    pub async fn set_settling_parameters(&self, time: u16, time_limit: u16) -> Result<(), Error> {
        let mut args = BytesMut::new();
        time.serialize(&mut args);
        time_limit.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 46, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  time: {:?}", time),
                    format!("  time_limit: {:?}", time_limit),
                ];
                format!(
                    "in call to NimbusCoreXDrive.SetSettlingParameters(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn is_axis_configured(&self) -> Result</* configured= */ bool, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 47, args.freeze()).await,
            || "in call to NimbusCoreXDrive.IsAxisConfigured()".to_string(),
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let configured = bool::deserialize(&mut stream)?;
        Ok(configured)
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreXDrive.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreXDrive.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreXDrive.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreXDrive.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreXDrive.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreXDrive.StructInfo(\n{}\n)",
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
pub enum EMotionprofile {
    EmotionprofileTrapezoidal = 0,
    EmotionprofileParabolic = 1,
    EmotionprofileStep = 2,
    EmotionprofileScurve = 3,
}

impl TryFrom<i32> for EMotionprofile {
    type Error = piglet_client::client::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(EMotionprofile::EmotionprofileTrapezoidal),
            1 => Ok(EMotionprofile::EmotionprofileParabolic),
            2 => Ok(EMotionprofile::EmotionprofileStep),
            3 => Ok(EMotionprofile::EmotionprofileScurve),

            _ => Err(ConnectionError(anyhow!(
                "Unknown EMotionprofile value {}",
                v
            ))),
        }
    }
}

impl PigletCodec for EMotionprofile {
    const TYPE_ID: u8 = 32;
}

impl PigletDeserialize for EMotionprofile {
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

impl PigletSerialize for EMotionprofile {
    fn serialize(&self, stream: &mut BytesMut) {
        stream.put_u8(Self::TYPE_ID);
        stream.put_u8(0);
        stream.put_u16_le(4);
        stream.put_i32_le(*self as i32);
    }
}

impl PigletCodec for MSlice<'_, EMotionprofile> {
    const TYPE_ID: u8 = 35;
}

impl PigletCodec for MVec<EMotionprofile> {
    const TYPE_ID: u8 = 35;
}

impl PigletDeserialize for MVec<EMotionprofile> {
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

impl PigletSerialize for MSlice<'_, EMotionprofile> {
    fn serialize(&self, bytes: &mut BytesMut) {
        bytes.put_u8(Self::TYPE_ID);
        bytes.put_u8(0);
        bytes.put_u16_le(4 * self.0.len() as u16);
        for v in self.0.as_ref() {
            bytes.put_i32_le(*v as i32);
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetVelAccReply {
    velocity: u32,
    acceleration: u32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetTravelLimitsReply {
    lower_limit: i32,
    upper_limit: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct GetSettlingParametersReply {
    time: u16,
    time_limit: u16,
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
