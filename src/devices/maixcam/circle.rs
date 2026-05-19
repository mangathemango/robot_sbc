use std::fmt::Display;

use glam::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct MaixcamCircle {
    pub position: Vec2,
    pub speed: f32,
    pub color: MaixcamCircleColor,
    pub kind: MaixcamCircleKind,
}

impl Display for MaixcamCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} circle at {:.2} moving at speed {:.2}",
            self.color, self.position, self.speed
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MaixcamCircleColor {
    #[default]
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaixcamCircleKind {
    Ring,
    #[default]
    Solid,
}
