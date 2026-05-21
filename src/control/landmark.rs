pub const LANDMARK_SCALE: f32 = 0.07;

use std::{f32::consts::{FRAC_PI_2, PI}};

use glam::Vec2;

use crate::math::Pose;

#[derive(Debug, Clone, Copy)]
pub enum Landmark {
    Start,
    QrZone,
    SourceZone,
    CentralRightCrossing,
    TemporaryStorageZone,
    UpperLeftTurn,
    FinalProcessingZone,
    UpperRightTurn,
    Custom(Pose)
}

impl Landmark {
    pub fn pose(&self) -> Pose {
        let normalized = match self {
            Landmark::Start => Pose {
                position: Vec2::new(0.0, 0.0),
                rotation: 0.0,
            },

            Landmark::QrZone => Pose {
                position: Vec2::new(-0.05, 0.33),
                rotation: 0.0,
            },

            Landmark::SourceZone => Pose {
                position: Vec2::new(-0.05, 0.70),
                rotation: 0.0,
            },

            Landmark::CentralRightCrossing => Pose {
                position: Vec2::new(-0.05, 0.50),
                rotation: PI,
            },

            Landmark::TemporaryStorageZone => Pose {
                position: Vec2::new(-0.95, 0.50),
                rotation: PI,
            },

            Landmark::UpperLeftTurn => Pose {
                position: Vec2::new(-0.95, 0.95),
                rotation: FRAC_PI_2,
            },

            Landmark::FinalProcessingZone => Pose {
                position: Vec2::new(-0.45, 0.95),
                rotation: FRAC_PI_2,
            },

            Landmark::UpperRightTurn => Pose {
                position: Vec2::new(-0.05, 0.95),
                rotation: 0.0,
            },

            Landmark::Custom(pose) => *pose
        };
        normalized.scale(LANDMARK_SCALE)
    }
}
