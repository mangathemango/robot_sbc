use glam::Vec2;

use crate::math::Pose;

#[derive(Debug, Clone, Copy)]
pub enum Landmark {
    Start,
    QrZone,
    SourceZone,
    SideIntersection,
    TemporaryStorageZone,
    FirstCornerTurn,
    FinalProcessingZone,
    SecondCornerTurn
}

impl Landmark {
    pub fn pose(&self) -> Pose {
        match self {
            Landmark::Start => Pose {
                position: Vec2::new(0.0, 0.0),
                rotation: 0.0,
            },
            Landmark::QrZone => Pose {
                position: Vec2::new(-0.10, 0.15),
                rotation: 0.0,
            },
            Landmark::SourceZone => Pose {
                position: Vec2::new(-0.05, 0.45),
                rotation: 0.0,
            },
            Landmark::SideIntersection => Pose {
                position: Vec2::new(-0.05, 0.30),
                rotation: 0.0,
            },
            Landmark::TemporaryStorageZone => Pose {
                position: Vec2::new(-0.57, 0.30),
                rotation: 0.0,
            },
            Landmark::FirstCornerTurn => Pose {
                position: Vec2::new(-0.57, 0.57),
                rotation: 0.0,
            },
            Landmark::FinalProcessingZone => Pose {
                position: Vec2::new(-0.30, 0.57),
                rotation: 0.0,
            },
            Landmark::SecondCornerTurn => Pose {
                position: Vec2::new(-0.05, 0.57),
                rotation: 0.0,
            },
        }
    }
}
