use glam::Vec2;

use crate::ROBOT;
use crate::math::{MecanumVelocities, Pose, Twist, utils::wrap_angle};
use std::time::Duration;

pub fn spawn_odometry_thread() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(100));
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(10) {
                continue;
            }
            last_update = now;

            ROBOT.lock_odometry_state().update(dt);
        }
    });
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OdometryState {
    pub twist: Twist,
    pub pose: Pose,
    pub initial_rotation: f32,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl OdometryState {
    pub fn new() -> Self {
        let gyro_state = ROBOT.gyro_state.load();
        let initial_rotation = if gyro_state.driver_is_connected {
            gyro_state.yaw
        } else {
            f32::NAN
        };

        Self {
            initial_rotation,
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.dt = dt;

        let stm32_state = ROBOT.stm32_state.load();
        let gyro_state = ROBOT.gyro_state.load();

        if self.initial_rotation.is_nan() {
            self.initial_rotation = gyro_state.yaw;
        }

        let [vfl, vfr, vrl, vrr] = stm32_state
            .actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.twist = MecanumVelocities::new(vfl, vfr, vrl, vrr).to_twist();

        let translation =
            (self.twist.linear * dt.as_secs_f32()).rotate(Vec2::from_angle(self.pose.rotation));

        self.pose.position += translation;
        self.pose.rotation = wrap_angle(gyro_state.yaw - self.initial_rotation);
    }
}
