use std::fmt::Display;

use crate::{ROBOT, control::actions::Action};

pub type LowerArm = LiftArm;

#[derive(Debug, Clone, Default)]
pub struct LiftArm {
    target_position: u16,
}

impl LiftArm {
    pub fn to_preset(position: ArmLiftPreset) -> Self {
        Self {
            target_position: position.to_position(),
        }
    }

    pub fn up() -> Self {
        Self::to_preset(ArmLiftPreset::Up)
    }

    pub fn to_storage() -> Self {
        Self::to_preset(ArmLiftPreset::Storage)
    }

    pub fn to_ground() -> Self {
        Self::to_preset(ArmLiftPreset::Ground)
    }

    pub fn to_stacked() -> Self {
        Self::to_preset(ArmLiftPreset::Stack)
    }
}

impl Action for LiftArm {
    fn start(&mut self) {
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_vertical_arm_position(self.target_position);
    }

    fn is_finished(&self) -> bool {
        self.target_position
            .abs_diff(ROBOT.get_stm32_state().vertical_arm_position)
            < 100
    }
}

impl Display for LiftArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lifting Arm to {}", self.target_position)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArmLiftPreset {
    Up,
    Storage,
    Ground,
    Stack,
}

impl ArmLiftPreset {
    pub fn to_position(&self) -> u16 {
        match self {
            ArmLiftPreset::Up => 10000,
            ArmLiftPreset::Storage => 6000,
            ArmLiftPreset::Ground => 1000,
            ArmLiftPreset::Stack => 2000,
        }
    }
}
