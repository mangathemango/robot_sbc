pub mod commands;
pub mod controller;
pub mod driver;
pub mod state;

use crate::devices::stm32::driver::Stm32Driver;
use crate::devices::stm32::state::Stm32State;
use crate::math::MecanumVelocities;
use commands::PiToStm32Command;

use std::sync::mpsc::Receiver;
use std::time::Duration;
const STM32_DOTENV_KEY: &str = "STM32_PATH";
const STM32_START_BYTE: u8 = 0x67;

pub fn spawn_stm32_thread(rx: Receiver<PiToStm32Command>) {
    std::thread::spawn(move || {
        let mut driver = Stm32Driver::new();
        let mut state = Stm32State::new();
        let mut last_update = std::time::Instant::now();
        let mut target_mecanum_velocities = MecanumVelocities::default();
        loop {
            let now = std::time::Instant::now();
            let dt = now - last_update;
            if dt < Duration::from_millis(20) {
                continue;
            }
            state.dt = now.duration_since(last_update);
            last_update = now;

            // 🟣 1. Handle outgoing commands
            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    PiToStm32Command::SetWheelTargetVelocities { velocities } => {
                        target_mecanum_velocities = MecanumVelocities::from_array(velocities.map(|v| v as f32));
                    }
                    PiToStm32Command::SetClawServoAngle { angle } => {
                        state.claw_servo_current_angle = angle;
                    }
                    PiToStm32Command::SetYawServoAngle { angle } => {
                        state.yaw_servo_current_angle = angle;
                    }
                    _ => (),
                }
                let _ = driver.send_command(cmd);
            }


            // 🔵 2. Handle incoming data
            match driver.try_read_frame() {
                Ok(Some(command)) => {
                    state.update(command);
                }
                Ok(None) => {}
                Err(_) => {
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }

            let current_mecanum_velocities = MecanumVelocities::from_array(
                state.actual_wheel_velocities.map(|v| v as f32),
            );
            let simulated_mecanum_velocities = current_mecanum_velocities
                .simulate_mecanum_response(target_mecanum_velocities, state.dt);
            state.actual_wheel_velocities =
                simulated_mecanum_velocities.to_array().map(|v| v as i16);
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}







