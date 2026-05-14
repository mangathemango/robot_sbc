use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action, devices::maixcam::color::MaixcamCircleColor};

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

    pub fn to_preset(target_position: ArmRotationPreset) -> Self {
        Self {
            target_angle: target_position.to_angle(),
            ..Default::default()
        }
    }

    pub fn middle() -> Self {
        Self::to_preset(ArmRotationPreset::Middle)
    }
    pub fn left() -> Self {
        Self::to_preset(ArmRotationPreset::Left)
    }
    pub fn right() -> Self {
        Self::to_preset(ArmRotationPreset::Right)
    }
    pub fn middle_storage() -> Self {
        Self::to_preset(ArmRotationPreset::MiddleStorage)
    }
    pub fn left_storage() -> Self {
        Self::to_preset(ArmRotationPreset::LeftStorage)
    }
    pub fn right_storage() -> Self {
        Self::to_preset(ArmRotationPreset::RightStorage)
    }
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

impl Display for RotateArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rotating Arm to {}", self.target_angle)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArmRotationPreset {
    #[default]
    Middle,
    Left,
    LeftPlacement(MaixcamCircleColor),
    Right,
    RightPlacement(MaixcamCircleColor),
    LeftStorage,
    MiddleStorage,
    RightStorage,
    Custom(u8),
}

impl ArmRotationPreset {
    pub fn to_angle(&self) -> u8 {
        match self {
            ArmRotationPreset::Middle => 60,
            ArmRotationPreset::Left => 120,
            ArmRotationPreset::Right => 0,
            ArmRotationPreset::LeftStorage => 50,
            ArmRotationPreset::MiddleStorage => 60,
            ArmRotationPreset::RightStorage => 70,
            ArmRotationPreset::LeftPlacement(color) => match color {
                MaixcamCircleColor::Green => ArmRotationPreset::Left.to_angle(),
                MaixcamCircleColor::Blue => ArmRotationPreset::Left.to_angle() + 11,
                MaixcamCircleColor::Red => ArmRotationPreset::Left.to_angle() - 11,
                MaixcamCircleColor::Unknown => ArmRotationPreset::Left.to_angle(),
            },
            ArmRotationPreset::RightPlacement(color) => match color {
                MaixcamCircleColor::Green => ArmRotationPreset::Right.to_angle(),
                MaixcamCircleColor::Blue => ArmRotationPreset::Right.to_angle() + 11,
                MaixcamCircleColor::Red => ArmRotationPreset::Right.to_angle() - 11,
                MaixcamCircleColor::Unknown => ArmRotationPreset::Right.to_angle(),
            },
            ArmRotationPreset::Custom(angle) => *angle,
        }
    }

    pub fn from_angle(angle: u8) -> Self {
        match angle {
            0 => ArmRotationPreset::Right,
            50 => ArmRotationPreset::LeftStorage,
            60 => ArmRotationPreset::Middle,
            70 => ArmRotationPreset::RightStorage,
            120 => ArmRotationPreset::Left,
            _ => ArmRotationPreset::Custom(angle),
        }
    }
}
