use std::sync::Arc;

use crate::{ROBOT, devices::stm32::commands::Stm32ToPiCommand};

/// A struct representing the current states polled from the Stm32
#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32State {
    pub driver_is_connected: bool,
    pub start_flag: bool,

    pub yaw_servo_current_angle: u8,
    pub claw_servo_current_angle: u8,
    // Movement
    pub actual_wheel_velocities: [i16; 4],
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl Stm32State {
    pub fn new() -> Stm32State {
        Stm32State::default()
    }

    pub fn update(&mut self, command: Stm32ToPiCommand) {
        match command {
            Stm32ToPiCommand::SendActualWheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
            }
            Stm32ToPiCommand::SetRunningFlag { running } => {
                self.start_flag = running != 0;
            }
        };
    }

    pub fn publish(&self) {
        ROBOT.stm32_state.store(Arc::new(self.clone()));
    }
}