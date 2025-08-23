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

pub struct NimbusCoreGripper {
    address: ObjectAddress,
    robot: Arc<RobotClient>,
}

impl NimbusCoreGripper {
    // version 0.2

    pub fn new(robot: &Arc<RobotClient>) -> Self {
        Self {
            address: ObjectAddress {
                module_id: 1,
                node_id: 1,
                object_id: 264,
            },
            robot: robot.clone(),
        }
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 1, args.freeze()).await,
            || "in call to NimbusCoreGripper.Initialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn initialize_smart(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 2, args.freeze()).await,
            || "in call to NimbusCoreGripper.InitializeSmart()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn park(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 3, args.freeze()).await,
            || "in call to NimbusCoreGripper.Park()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pick_up_plate(
        &self,

        x_position: i32,
        y_position: i32,
        z_position: i32,
        grip_angle: i32,
        plate_width: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
        final_z_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        grip_angle.serialize(&mut args);
        plate_width.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        final_z_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 4, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  grip_angle: {:?}", grip_angle),
                    format!("  plate_width: {:?}", plate_width),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                    format!("  final_z_position: {:?}", final_z_position),
                ];
                format!(
                    "in call to NimbusCoreGripper.PickUpPlate(\n{}\n)",
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
        y_position: i32,
        z_position: i32,
        grip_angle: i32,
        plate_width: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
        final_z_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        grip_angle.serialize(&mut args);
        plate_width.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        final_z_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 5, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  grip_angle: {:?}", grip_angle),
                    format!("  plate_width: {:?}", plate_width),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                    format!("  final_z_position: {:?}", final_z_position),
                ];
                format!(
                    "in call to NimbusCoreGripper.DropPlate(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn pick_up_plate_force(
        &self,

        x_position: i32,
        y_position: i32,
        z_position: i32,
        grip_angle: i32,
        plate_width: i32,
        grip_open_distance: i32,
        force: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
        final_z_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        grip_angle.serialize(&mut args);
        plate_width.serialize(&mut args);
        grip_open_distance.serialize(&mut args);
        force.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        final_z_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 6, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  grip_angle: {:?}", grip_angle),
                    format!("  plate_width: {:?}", plate_width),
                    format!("  grip_open_distance: {:?}", grip_open_distance),
                    format!("  force: {:?}", force),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                    format!("  final_z_position: {:?}", final_z_position),
                ];
                format!(
                    "in call to NimbusCoreGripper.PickUpPlateForce(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn drop_plate_force(
        &self,

        x_position: i32,
        y_position: i32,
        z_position: i32,
        grip_angle: i32,
        plate_width: i32,
        grip_open_distance: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
        final_z_position: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        grip_angle.serialize(&mut args);
        plate_width.serialize(&mut args);
        grip_open_distance.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        final_z_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 7, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  grip_angle: {:?}", grip_angle),
                    format!("  plate_width: {:?}", plate_width),
                    format!("  grip_open_distance: {:?}", grip_open_distance),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                    format!("  final_z_position: {:?}", final_z_position),
                ];
                format!(
                    "in call to NimbusCoreGripper.DropPlateForce(\n{}\n)",
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

        x_position: i32,
        y_position: i32,
        z_position: i32,
        wrist_angle: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        wrist_angle.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 8, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  wrist_angle: {:?}", wrist_angle),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                ];
                format!(
                    "in call to NimbusCoreGripper.MoveToPosition(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn move_to_position_zw(
        &self,

        x_position: i32,
        y_position: i32,
        z_position: i32,
        wrist_angle: i32,
        channel_traverse_height: i32,
        gripper_traverse_height: i32,
    ) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        z_position.serialize(&mut args);
        wrist_angle.serialize(&mut args);
        channel_traverse_height.serialize(&mut args);
        gripper_traverse_height.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 9, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                    format!("  z_position: {:?}", z_position),
                    format!("  wrist_angle: {:?}", wrist_angle),
                    format!("  channel_traverse_height: {:?}", channel_traverse_height),
                    format!("  gripper_traverse_height: {:?}", gripper_traverse_height),
                ];
                format!(
                    "in call to NimbusCoreGripper.MoveToPositionZW(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn xy_move_absolute(&self, x_position: i32, y_position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        x_position.serialize(&mut args);
        y_position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 10, args.freeze()).await,
            || {
                let parameters = vec![
                    format!("  x_position: {:?}", x_position),
                    format!("  y_position: {:?}", y_position),
                ];
                format!(
                    "in call to NimbusCoreGripper.XYMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn x_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 11, args.freeze()).await,
            || "in call to NimbusCoreGripper.XInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn x_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 12, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.XMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn x_move_relative(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 13, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.XMoveRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 14, args.freeze()).await,
            || "in call to NimbusCoreGripper.YInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 15, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.YMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn y_move_relative(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 16, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.YMoveRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 17, args.freeze()).await,
            || "in call to NimbusCoreGripper.ZInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 18, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.ZMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_move_relative(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 19, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.ZMoveRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn z_up(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 20, args.freeze()).await,
            || "in call to NimbusCoreGripper.ZUp()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn w_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 21, args.freeze()).await,
            || "in call to NimbusCoreGripper.WInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn w_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 22, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.WMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn w_move_relative(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 23, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.WMoveRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn g_initialize(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 24, args.freeze()).await,
            || "in call to NimbusCoreGripper.GInitialize()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn g_move_absolute(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 25, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.GMoveAbsolute(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn g_move_relative(&self, position: i32) -> Result<(), Error> {
        let mut args = BytesMut::new();
        position.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 26, args.freeze()).await,
            || {
                let parameters = vec![format!("  position: {:?}", position)];
                format!(
                    "in call to NimbusCoreGripper.GMoveRelative(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn get_desired_position(&self) -> Result<GetDesiredPositionReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 0, 27, args.freeze()).await,
            || "in call to NimbusCoreGripper.GetDesiredPosition()".to_string(),
        )?;

        if count != 5 {
            return Err(ConnectionError(anyhow!("Expected 5 values, not {}", count)));
        }
        let x_position = i32::deserialize(&mut stream)?;
        let y_position = i32::deserialize(&mut stream)?;
        let z_position = i32::deserialize(&mut stream)?;
        let w_position = i32::deserialize(&mut stream)?;
        let g_position = i32::deserialize(&mut stream)?;
        Ok(GetDesiredPositionReply {
            x_position,
            y_position,
            z_position,
            w_position,
            g_position,
        })
    }

    pub async fn initialize_zwy(&self, smart: bool) -> Result<(), Error> {
        let mut args = BytesMut::new();
        smart.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 28, args.freeze()).await,
            || {
                let parameters = vec![format!("  smart: {:?}", smart)];
                format!(
                    "in call to NimbusCoreGripper.InitializeZWY(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn w_hold(&self) -> Result<(), Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 29, args.freeze()).await,
            || "in call to NimbusCoreGripper.WHold()".to_string(),
        )?;

        if count != 0 {
            return Err(ConnectionError(anyhow!("Expected 0 values, not {}", count)));
        }
        Ok(())
    }

    pub async fn w_hold_2(&self, tips: bool) -> Result</* z_safe= */ bool, Error> {
        let mut args = BytesMut::new();
        tips.serialize(&mut args);
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 1, 3, 30, args.freeze()).await,
            || {
                let parameters = vec![format!("  tips: {:?}", tips)];
                format!(
                    "in call to NimbusCoreGripper.WHold2(\n{}\n)",
                    parameters.join("\n")
                )
            },
        )?;

        if count != 1 {
            return Err(ConnectionError(anyhow!("Expected 1 values, not {}", count)));
        }
        let z_safe = bool::deserialize(&mut stream)?;
        Ok(z_safe)
    }

    pub async fn object_info(&self) -> Result<ObjectInfoReply, Error> {
        let mut args = BytesMut::new();
        let (count, mut stream) = with_context(
            self.robot.act(&self.address, 0, 0, 1, args.freeze()).await,
            || "in call to NimbusCoreGripper.ObjectInfo()".to_string(),
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
                    "in call to NimbusCoreGripper.MethodInfo(\n{}\n)",
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
                    "in call to NimbusCoreGripper.SubObjectInfo(\n{}\n)",
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
            || "in call to NimbusCoreGripper.InterfaceDescriptors()".to_string(),
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
                    "in call to NimbusCoreGripper.EnumInfo(\n{}\n)",
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
                    "in call to NimbusCoreGripper.StructInfo(\n{}\n)",
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
pub struct GetDesiredPositionReply {
    x_position: i32,
    y_position: i32,
    z_position: i32,
    w_position: i32,
    g_position: i32,
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
