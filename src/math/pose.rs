use glam::Vec2;

/// A struct representing where an object is and where it's facing
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    position: Vec2,
    rotation: f32 // in radians
}

impl Pose {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Pose {position, rotation}
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