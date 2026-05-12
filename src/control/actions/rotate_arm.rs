use std::time::Duration;

use crate::{ROBOT, control::actions::Action};

#[derive(Clone, Copy, Debug, Default)]
pub struct RotateArm {
    pub initial_angle: u8,
    pub target_angle: u8,
    pub elapsed_time: Duration,
}

impl RotateArm {
    pub fn to_angle(target_angle: u8) -> Self {
        Self {
            target_angle,
            ..Default::default()
        }
    }

    pub fn to(target_position: RotatePosition) -> Self {
        Self {
            target_angle: target_position.to_angle(),
            ..Default::default()
        }
    }

    pub fn middle() -> Self         { Self::to(RotatePosition::Middle) }
    pub fn left() -> Self           { Self::to(RotatePosition::Left) }
    pub fn right() -> Self          { Self::to(RotatePosition::Right) }
    pub fn middle_storage() -> Self { Self::to(RotatePosition::MiddleStorage) }
    pub fn left_storage() -> Self   { Self::to(RotatePosition::LeftStorage) }
    pub fn right_storage() -> Self  { Self::to(RotatePosition::RightStorage) }
}

impl Action for RotateArm {
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

#[derive(Debug, Clone, Copy, Default)]
enum RotatePosition {
    #[default]
    Middle,
    Left,
    Right,
    LeftStorage,
    MiddleStorage,
    RightStorage,
    Custom(u8),
}

impl RotatePosition {
    pub fn to_angle(&self) -> u8 {
        match self {
            RotatePosition::Middle =>        60,
            RotatePosition::Left =>          120,
            RotatePosition::Right =>         0,
            RotatePosition::LeftStorage =>   50,
            RotatePosition::MiddleStorage => 60,
            RotatePosition::RightStorage =>  70,
            RotatePosition::Custom(angle) => *angle,
        }
    }
}
