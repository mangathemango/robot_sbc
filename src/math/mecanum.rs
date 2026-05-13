use std::time::Duration;

use crate::math::Twist;

/// A struct representing the velocities of a set of 4 mecanum wheels
#[derive(Debug, Clone, Copy, Default)]
pub struct MecanumVelocities {
    /// Front Left wheel velocity
    pub vfl: f32, 
    /// Front Right wheel velocity
    pub vfr: f32,
    /// Rear Left wheel velocity
    pub vrl: f32,
    /// Rear Right wheel velocity
    pub vrr: f32,
}

impl std::fmt::Display for MecanumVelocities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(vfl: {:.3}, vfr: {:.3}\nvrl: {:.3}, vrr: {:.3})",
            self.vfl, self.vfr, self.vrl, self.vrr
        )
    }
}

impl MecanumVelocities {
    pub fn new(vfl: f32, vfr: f32, vrl: f32, vrr: f32) -> Self {
        MecanumVelocities { vfl, vfr, vrl, vrr }
    }

    /// Clamp all mecanum velocities to [-1.0, 1.0]
    pub fn normalize(&mut self) -> Self {
        let max = self
            .vfl
            .abs()
            .max(self.vfr.abs())
            .max(self.vrl.abs())
            .max(self.vrr.abs())
            .max(1.0);
        self.vfl /= max;
        self.vfr /= max;
        self.vrl /= max;
        self.vrr /= max;
        *self
    }

    pub fn from_twist(t: Twist) -> Self {
        let vx = t.linear.x;
        let vy = t.linear.y;
        let omega = t.omega;

        let vfl = vy - vx - omega;
        let vfr = vy + vx + omega;
        let vrl = vy + vx - omega;
        let vrr = vy - vx + omega;

        MecanumVelocities { vfl, vfr, vrl, vrr }
    }

    pub fn to_twist(&self) -> Twist {
        Twist::from_mecanum_velocities(*self)
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.vfl, self.vfr, self.vrl, self.vrr]
    }

    pub fn from_array(array: [f32; 4]) -> Self {
        Self { vfl: array[0], vfr: array[1], vrl: array[2], vrr: array[3] }
    }

    pub fn simulate_mecanum_response(
        &self,
        target: MecanumVelocities,
        dt: Duration,
    ) -> Self {
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


        let accel = 10000.0;
        let max_delta =
            accel * dt.as_secs_f32();
        let max_noise = 0.0;

        // Simulated wheel response
        let vfl = approach(
            self.vfl,
            target.vfl,
            max_delta,
        ) + noise(max_noise);

        let vfr = approach(
            self.vfr,
            target.vfr,
            max_delta,
        ) + noise(max_noise);

        let vrl = approach(
            self.vrl,
            target.vrl,
            max_delta,
        ) + noise(max_noise);

        let vrr = approach(
            self.vrr,
            target.vrr,
            max_delta,
        ) + noise(max_noise);

        // Convert back to twist
        MecanumVelocities {
            vfl,
            vfr,
            vrl,
            vrr
        }
    }
}
