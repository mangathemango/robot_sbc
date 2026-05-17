use std::{fmt::Display};

use crate::{ROBOT, control::actions::Action, devices::maixcam::circle::MaixcamCircleColor};

pub type RetractArm = ExtendArm;

#[derive(Debug, Clone, Default)]
pub struct ExtendArm {
    target_position: u16,
}

impl ExtendArm {
    pub fn to_position(position: u16) -> Self {
        ExtendArm { target_position: position }
    }

    pub fn to_preset(preset: ArmExtendPreset) -> Self {
        Self::to_position(preset.to_position())
    }

    pub fn back() -> Self {
        Self::to_preset(ArmExtendPreset::Back)
    }
}

impl Action for ExtendArm {
    fn start(&mut self) {
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_horizontal_arm_position(self.target_position);
    }

    fn is_finished(&self) -> bool {
        self.target_position.abs_diff(ROBOT.get_stm32_state().horizontal_arm_position) < 100
    }
}

impl Display for ExtendArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Extending Arm to {}", self.target_position)
    }
}

pub enum ArmExtendPreset {
    Back,
    Storage(MaixcamCircleColor),
    Placement(MaixcamCircleColor)
}

impl ArmExtendPreset {
    pub fn to_position(&self) -> u16 {
        match self {
            Self::Back => 0,
            Self::Storage(color) => {
                match color {
                    MaixcamCircleColor::Green => 1000,
                    MaixcamCircleColor::Blue => 2000,
                    MaixcamCircleColor::Red => 3000
                }
            },
            Self::Placement(color) => {
                match color {
                    MaixcamCircleColor::Blue => 8000,
                    MaixcamCircleColor::Green => 2000,
                    MaixcamCircleColor::Red => 8000
                }
            }
        }
    }
}