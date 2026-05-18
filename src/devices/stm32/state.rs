use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    ROBOT,
    devices::stm32::{command::Stm32Command, message::Stm32Message},
};

/// A struct representing the current states polled from the Stm32
#[derive(Debug, Default, Clone)]
pub struct Stm32State {
    pub driver_is_connected: bool,
    pub start_flag: Arc<AtomicBool>,

    pub yaw_servo_current_angle: u8,
    pub claw_servo_current_angle: u8,

    pub horizontal_arm_position: u16,
    pub vertical_arm_position: u16,

    pub last_command: Stm32Command,
    pub last_message: Stm32Message,
    pub log_msg: String,
    // Movement
    pub target_wheel_velocities: [i16; 4],
    pub actual_wheel_velocities: [i16; 4],
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl Stm32State {
    pub fn new() -> Stm32State {
        Stm32State::default()
    }

    pub fn update_command(&mut self, command: Stm32Command) {
        match command {
            Stm32Command::SetWheelTargetVelocities { velocities } => {
                self.target_wheel_velocities = velocities;
            }
            Stm32Command::SetClawServoAngle { angle } => {
                self.claw_servo_current_angle = angle;
            }
            Stm32Command::SetYawServoAngle { angle } => {
                self.yaw_servo_current_angle = angle;
            }
            Stm32Command::SetVerticalArmPosition { position } => {
                self.vertical_arm_position = position
            }
            Stm32Command::SetHorizontalArmPosition { position } => {
                self.horizontal_arm_position = position
            }
            _ => (),
        }
    }

    pub fn update_message(&mut self, message: Stm32Message) {
        match message.clone() {
            Stm32Message::Log { msg } => self.log_msg = msg,
            Stm32Message::WheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
            }
            Stm32Message::Key1 => {
                self.start_flag.store(true, Ordering::Relaxed);
            }
            Stm32Message::HorizontalArmPosition { position } => {
                self.horizontal_arm_position = position
            }
            Stm32Message::VerticalArmPosition { position } => self.vertical_arm_position = position,
        };
        self.last_message = message.clone();
    }

    pub fn publish(&self) {
        ROBOT.set_stm32_state(self.clone());
    }
}

impl Display for Stm32State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Last command: {:?}\n({})\n\nLast message: {:?}\n({})\n\nFront Wheels: {:06}   {:06}\nBack Wheels:  {:06}   {:06}\nYaw servo: {}\nClaw servo: {}\nVertical Arm: {}\nHorizontal Arm: {}\nLog: {}\ndt: {:?}",
            self.last_command, self.last_command.to_bytes_string(),
            self.last_message, self.last_message.to_bytes_string(),
            self.yaw_servo_current_angle,
            self.claw_servo_current_angle,
            self.vertical_arm_position,
            self.horizontal_arm_position,
            self.actual_wheel_velocities[0],
            self.actual_wheel_velocities[1],
            self.actual_wheel_velocities[2],
            self.actual_wheel_velocities[3],
            self.log_msg,
            self.dt
        )
    }
}
