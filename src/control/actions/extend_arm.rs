use std::{fmt::Display};

use crate::{ROBOT, control::actions::Action};

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
    Storage,
    PlacementStraight,
    PlacementDiagonal
}

impl ArmExtendPreset {
    pub fn to_position(&self) -> u16 {
        match self {
            ArmExtendPreset::Back => 0,
            ArmExtendPreset::Storage => 100,
            ArmExtendPreset::PlacementStraight => 1000,
            ArmExtendPreset::PlacementDiagonal => 8000,
        }
    }
}