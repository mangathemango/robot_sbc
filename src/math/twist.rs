use crate::math::MecanumVelocities;
use glam::Vec2;
use std::time::Duration;
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
    pub const ZERO: Self = Twist {linear: Vec2::ZERO, omega: 0.0};

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


    pub fn simulate_mecanum_response(
        &mut self,
        target: MecanumVelocities,
        dt: Duration,
    ) {
        fn approach(
            current: f32,
            target: f32,
            max_delta: f32,
        ) -> f32 {
            let delta = target - current;

            if delta > max_delta {
                current + max_delta
            } else if delta < -max_delta {
                current - max_delta
            } else {
                target
            }
        }

        fn noise(amount: f32) -> f32 {
            (rand::random::<f32>() - 0.5)
                * 2.0
                * amount
        }

        // Convert current twist → wheel velocities
        let current =
            MecanumVelocities::from_twist(*self);

        let accel = 0.5;
        let max_delta =
            accel * dt.as_secs_f32();

        // Simulated wheel response
        let vfl = approach(
            current.vfl,
            target.vfl,
            max_delta,
        ) + noise(0.01);

        let vfr = approach(
            current.vfr,
            target.vfr,
            max_delta,
        ) + noise(0.01);

        let vrl = approach(
            current.vrl,
            target.vrl,
            max_delta,
        ) + noise(0.01);

        let vrr = approach(
            current.vrr,
            target.vrr,
            max_delta,
        ) + noise(0.01);

        // Convert back to twist
        *self = MecanumVelocities {
            vfl,
            vfr,
            vrl,
            vrr,
        }
        .to_twist();
    }
}

