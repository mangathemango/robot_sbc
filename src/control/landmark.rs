const LANDMARK_SCALING: f32 = 0.6;

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
                rotation: 0.0,
            },

            Landmark::TemporaryStorageZone => Pose {
                position: Vec2::new(-0.95, 0.50),
                rotation: 0.0,
            },

            Landmark::UpperLeftTurn => Pose {
                position: Vec2::new(-0.95, 0.95),
                rotation: 0.0,
            },

            Landmark::FinalProcessingZone => Pose {
                position: Vec2::new(-0.45, 0.95),
                rotation: 0.0,
            },

            Landmark::UpperRightTurn => Pose {
                position: Vec2::new(-0.05, 0.95),
                rotation: 0.0,
            },
        };
        normalized.scale(LANDMARK_SCALING)
    }
}
