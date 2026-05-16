use std::sync::Arc;

use crate::{ROBOT, devices::stm32::message::Stm32Message};

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

    pub fn update(&mut self, command: Stm32Message) {
        match command {
            Stm32Message::Log { msg } => self.log_msg = msg,
            Stm32Message::WheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
            }
            Stm32Message::Key1 => {
                self.start_flag = !self.start_flag;
            }
            Stm32Message::HorizontalArmPosition { position } => {
                self.horizontal_arm_position = position
            }
            Stm32Message::VerticalArmPosition { position } => self.vertical_arm_position = position,
        };
    }

    pub fn publish(&self) {
        ROBOT.set_stm32_state(self.clone());
    }
}
