use std::f32::consts::PI;

use glam::Vec2;

/// A struct representing where an object is and where it's facing
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub position: Vec2,
    pub rotation: f32, // in radians
}

impl std::fmt::Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "((x: {:.3}, y: {:.3}), θ: {:.2}π rad ({:.1}°))",
            self.position.x,
            self.position.y,
            self.rotation / PI,
            self.rotation.to_degrees()
        )
    }
}

impl Pose {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Pose { position, rotation }
    }

    pub fn forward(&self) -> Vec2 {
        Vec2::from_angle(self.rotation)
    }
    pub fn right(&self) -> Vec2 {
        self.forward().perp()
    }
    pub fn back(&self) -> Vec2 {
        -self.forward()
    }
    pub fn left(&self) -> Vec2 {
        -self.right()
    }
}
