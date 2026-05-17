use core::fmt;
use std::{default, fmt::Display};

use glam::Vec2;

use crate::control::actions::{extend_arm::ArmExtendPreset};

#[derive(Debug, Default, Clone, Copy)]
pub struct MaixcamCircle {
    pub position: Vec2,
    pub color: MaixcamCircleColor,
    pub kind: MaixcamCircleKind,
}

impl Display for MaixcamCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} circle at {:.2}",self.color, self.kind, self.position)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MaixcamCircleColor {
    #[default]
    Unknown,
    Red,
    Green,
    Blue,
}

impl MaixcamCircleColor {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => MaixcamCircleColor::Red,
            2 => MaixcamCircleColor::Green,
            3 => MaixcamCircleColor::Blue,
            _ => MaixcamCircleColor::Unknown,
        }
    }

    pub fn placement_arm_extension(&self) -> ArmExtendPreset {
        match self {
            MaixcamCircleColor::Green => ArmExtendPreset::PlacementStraight,
            _ => ArmExtendPreset::PlacementDiagonal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaixcamCircleKind {
    Ring,
    #[default]
    Solid
}