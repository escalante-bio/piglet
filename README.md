# piglet

`piglet` is a Rust library designed to control IP-based Hamilton robots.

This project is an independent, open-source initiative and is not affiliated with, endorsed by, or
supported by Hamilton. Use it at your own risk.

[![Crates.io][crates-badge]][crates-url]
[![Apache 2.0 licensed][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/piglet.svg
[crates-url]: https://crates.io/crates/piglet
[license-badge]: https://img.shields.io/crates/l/piglet.svg?color=orange
[license-url]: https://github.com/escalante-bio/piglet/blob/main/LICENSE.txt

## Compatibility

While `piglet` has been primarily tested with a Nimbus HD (2021), it is easily extendable to the
ML Prep and other variants of the Nimbus platform. STAR and Vantage robots are significantly
different and fall outside the current scope of this project.

## Current state

* [ ] Test extensively and flesh out the missing pieces
* [ ] Generate code for other Nimbuses and iSWAP
* [ ] Generate code for ML Preps
* [ ] Add support for discovering robots on the network

## Example usage

```rust
use piglet::RobotClient;
use piglet::nimbus_hd_1_0::{
    nimbus_core::NimbusCore, nimbus_core_door_lock::NimbusCoreDoorLock,
    nimbus_core_hd_deck::NimbusCoreHdDeck, nimbus_core_pipette::NimbusCorePipette,
};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        anyhow::bail!("Usage: {} <ip>", args[0]);
    }

    println!("Connecting to {}...", args[1]);
    let robot = Arc::new(RobotClient::connect(&args[1]).await?);
    let core = NimbusCore::new(&robot);
    let deck = NimbusCoreHdDeck::new(&robot);
    let pipette = NimbusCorePipette::new(&robot);
    let lock = NimbusCoreDoorLock::new(&robot);

    lock.lock_door().await?;
    core.method_begin().await?;
    println!("{:?}", deck.get_track_sensor_states().await?);
    println!("{:?}", pipette.get_tip_and_needle_types().await?);
    for channel in 1..=8 {
        pipette
            .set_channel_configuration(channel, vec![1, 3, 4], vec![true, false, false])
            .await?;
    }

    // Coordinate system when facing the machine:
    // -x is left
    // -y is towards you
    // -z is down
    // 100 units is 1mm
    // channel 1 is the far channel, channel 8 is the close channel

    pipette.pickup_gripper_tool(
        /* x= */ 55735,
        /* y_position_1_st_channel= */ -26482,
        /* y_position_2_nd_channel= */ -29403,
        /* traverse_height= */ 18600,
        /* z_start_position= */ 17756,
        /* z_stop_position= */ 16756,
        /* tip_type= */ 14,
        /* first_channel_number= */ 7,
        /* second_channel_number= */ 8,
        /* tool_width= */ 200,
    ).await?;

    pipette.z_seek_obstacle(
        /* tips_used= */ vec![0, 0, 0, 0, 0, 0, 1, 0],
        /* x_position= */ vec![48086, 48086, 48086, 48086, 48086, 48086, 48086, 48086],
        /* y_position= */ vec![-29150, -29150, -29150, -29150, -29150, -29150, -29150, -29150],
        /* traverse_height= */ 18600,
        /* obstacle_seek_height= */ vec![10943, 10943, 10943, 10943, 10943, 10943, 10943, 10943],
        /* z_min_position= */ vec![10543, 10543, 10543, 10543, 10543, 10543, 10543, 10543],
        /* z_final= */ 18600,
        /* seek_speed= */ vec![12869, 12869, 12869, 12869, 12869, 12869, 12869, 12869],
    ).await?;

    pipette.pickup_plate(
        /* x_position= */ 48086,
        /* y_plate_center_position= */ -29150,
        /* y_plate_width= */ 8120,
        /* y_open_position= */ 8720,
        /* y_grip_speed= */ 27780,
        /* y_grip_strength= */ 250,
        /* traverse_height= */ 18600,
        /* z_grip_height= */ 10543,
        /* z_final= */ 18600,
        /* z_speed= */ 12869,
    ).await?;

    if !pipette.is_core_gripper_plate_gripped().await? {
        anyhow::bail!("Expected to grip a plate");
    }

    pipette.drop_plate(
        /* x_position= */ 48086,
        /* x_acceleration= */ 4,
        /* y_plate_center_position= */ -29150,
        /* y_open_position= */ 8720,
        /* traverse_height= */ 18600,
        /* z_drop_height= */ 10543,
        /* z_press_distance= */ 0,
        /* z_final= */ 18600,
        /* z_speed= */ 12869,
    ).await?;

    pipette.drop_gripper_tool(
        /* x_position= */ 55735,
        /* y_position_1_st_channel= */ -26482,
        /* y_position_2_nd_channel= */ -29403,
        /* traverse_height= */ 18600,
        /* z_start_position= */ 15756,
        /* z_stop_position= */ 14756,
        /* z_final= */ 18600,
        /* first_channel_number= */ 7,
        /* second_channel_number= */ 8,
    ).await?;

    pipette
        .move_to_position(
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            55361,
            vec![-161, -4558, -8955, -13352, -17749, -22146, -26543, -30940],
            vec![23550, 23550, 23550, 23550, 23550, 23550, 23550, 23550],
        )
        .await?;
    lock.unlock_door().await?;

    Ok(())
}
```

## Generating robot APIs

Hamilton robots offer an introspection API that allows for dynamic discovery of all available calls.
The `piglet_codegen` tool leverages this to generate type-safe Rust code for any robot it's pointed
at.

For example, the `piglet_generated/src/nimbus_hd_1_0` folder was generated by pointing the generator
at our Nimbus's IP address.

To generate API bindings:

1. **Build `piglet_codegen`:**
   ```bash
   cargo build --release --bin piglet_codegen
   ```
2. **Run the generator:** Point it to your robot's IP address and specify a module name
   ```bash
   ./target/release/piglet_codegen <robot ip and port> <module name>
   ```

As an example, the
[`nimbus_hd_1_0`](https://github.com/escalante-bio/piglet/tree/main/piglet_generated/src/nimbus_hd_1_0)
folder in this repository was generated from our Nimbus HD by running
`piglet_codegen 172.31.255.3:2000 nimbus_hd_1_0`
