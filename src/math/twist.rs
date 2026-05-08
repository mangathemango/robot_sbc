use crate::math::MecanumVelocities;
use glam::Vec2;
#[derive(Debug, Clone, Copy, Default)]
pub struct Twist {
    pub linear: Vec2, // (vx, vy)
    pub omega: f32,
}

impl std::fmt::Display for Twist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "((vx: {:.3}, vy: {:.3}), ω: {:.3})",
            self.linear.x, self.linear.y, self.omega
        )
    }
}

impl Twist {
    pub fn new(linear: Vec2, omega: f32) -> Self {
        Self { linear, omega }
    }

    pub fn from_mecanum_velocities(v: MecanumVelocities) -> Self {
        let vx = (- v.vfl + v.vfr + v.vrl - v.vrr) / 4.0;

        let vy = (v.vfl + v.vfr + v.vrl + v.vrr) / 4.0;

        let omega = (-v.vfl + v.vfr - v.vrl + v.vrr) / 4.0;

        Self {
            linear: Vec2::new(vx, vy),
            omega,
        }
    }

    pub fn to_mecanum_velocities(&self) -> MecanumVelocities {
        MecanumVelocities::from_twist(*self)
    }
}
