use std::sync::Arc;

use crate::{ROBOT, devices::stm32::commands::Stm32ToPiCommand};

/// A struct representing the current states polled from the Stm32
#[derive(Debug, Default, Clone)]
pub struct Stm32State {
    pub driver_is_connected: bool,
    pub start_flag: bool,

    pub yaw_servo_current_angle: u8,
    pub claw_servo_current_angle: u8,

    pub horizontal_arm_position: u16,
    pub vertical_arm_position: u16,

    pub log_msg: String,
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
            Stm32ToPiCommand::Log { msg } => {
                self.log_msg = msg
            }
            Stm32ToPiCommand::SendActualWheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
            }
            Stm32ToPiCommand::SetRunningFlag => {
                self.start_flag = !self.start_flag;
            }
            Stm32ToPiCommand::SendHorizontalArmPosition { position } => {
                self.horizontal_arm_position = position
            }
            Stm32ToPiCommand::SendVerticalArmPosition { position } => {
                self.vertical_arm_position = position
            }
        };
    }

    pub fn publish(&self) {
        ROBOT.stm32_state.store(Arc::new(self.clone()));
    }
}