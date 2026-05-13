use std::{fmt::Display, time::Duration};

use crate::{
    ROBOT,
    control::{ControllerState, actions::Action},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct RotateClaw {
    pub initial_angle: u8,
    pub target_angle: u8,
    pub elapsed_time: Duration,
}

impl RotateClaw {
    pub fn to(target_position: ClawPosition) -> Self {
        Self {
            target_angle: target_position.to_angle(),
            ..Default::default()
        }
    }
}

#[allow(unused_variables)]
impl Action for RotateClaw {
    fn start(&mut self) {
        self.initial_angle = ROBOT.stm32_state.load().yaw_servo_current_angle;
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_yaw_servo(self.target_angle);
    }

    fn update(&mut self, dt: Duration) {
        self.elapsed_time += dt
    }

    fn is_finished(&self) -> bool {
        self.elapsed_time
            > Duration::from_millis(self.target_angle.abs_diff(self.initial_angle) as u64 * 20)
    }

    fn stop(&mut self) {}

    fn current_action(&self) -> &dyn Action {
        self
    }
}

impl Display for RotateClaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rotate Claw to {}", self.target_angle)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClawPosition {
    #[default]
    Open,
    SoftOpen,
    Close,
    Custom(u8),
}

impl ClawPosition {
    pub fn to_angle(&self) -> u8 {
        match self {
            ClawPosition::Open => 30,
            ClawPosition::SoftOpen => 120,
            ClawPosition::Close => 180,
            ClawPosition::Custom(angle) => *angle,
        }
    }
}
