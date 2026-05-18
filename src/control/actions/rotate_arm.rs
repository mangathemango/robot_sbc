use std::{fmt::Display, time::Duration};

use crate::{ROBOT, control::actions::Action, devices::maixcam::circle::MaixcamCircleColor};

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
        Self::to_angle(target_position.to_angle())
    }

    pub fn to_storage(color: MaixcamCircleColor) -> Self {
        Self::to_preset(ArmRotationPreset::Storage(color))
    }

    
    pub fn to_placement(color: MaixcamCircleColor) -> Self {
        Self::to_preset(ArmRotationPreset::Placement(color))
    }
    
    pub fn idle() -> Self {
        Self::to_preset(ArmRotationPreset::Idle)
    }
}

impl Action for RotateArm {
    fn start(&mut self) {
        self.initial_angle = ROBOT.get_stm32_state().yaw_servo_current_angle;
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
}

impl Display for RotateArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rotating Arm to {}", self.target_angle)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArmRotationPreset {
    #[default]
    Idle,
    Storage(MaixcamCircleColor),
    Placement(MaixcamCircleColor),
    Calibration,

}

impl ArmRotationPreset {
    pub fn to_angle(&self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Storage(color) => {
                match color {
                    MaixcamCircleColor::Blue => 10,
                    MaixcamCircleColor::Green => 20,
                    MaixcamCircleColor::Red => 30,
                }
            },
            Self::Placement(color) => {
                match color {
                    MaixcamCircleColor::Blue => 40,
                    MaixcamCircleColor::Green => 50,
                    MaixcamCircleColor::Red => 60
                }
            }
            Self::Calibration => 90
        }
    }

    pub fn from_angle(angle: u8) -> Self {
        match angle {
            _ => Self::Idle,
        }
    }
}
