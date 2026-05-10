use glam::Vec2;

use crate::math::Pose;

#[derive(Debug, Clone, Copy)]
pub enum Landmark {
    SourceZone,
    TemporaryStorageZone,
    FinalProcessingZone,
}

impl Landmark {
    pub fn pose(&self) -> Pose {
        match self {
            Landmark::SourceZone => Pose {
                position: Vec2::new(-0.05, 0.45),
                rotation: 0.0,
            },
            Landmark::TemporaryStorageZone => Pose {
                position: Vec2::new(-0.57, 0.30),
                rotation: 0.0,
            },
            Landmark::FinalProcessingZone => Pose {
                position: Vec2::new(-0.05, 0.45),
                rotation: 0.0,
            },
        }
    }
}
