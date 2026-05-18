use glam::Vec2;

use crate::ROBOT;
use crate::control::landmark::{self, Landmark};
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
    pub gyro_offset: f32,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl OdometryState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, dt: Duration) {
        self.dt = dt;

        let stm32_state = ROBOT.get_stm32_state();
        let gyro_state = ROBOT.get_gyro_state();

        if self.gyro_offset.is_nan() {
            self.gyro_offset = gyro_state.yaw;
        }

        let [vfl, vfr, vrl, vrr] = stm32_state
            .actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.twist = MecanumVelocities::new(vfl, vfr, vrl, vrr).to_twist();

        let translation =
            (self.twist.linear * dt.as_secs_f32()).rotate(Vec2::from_angle(self.pose.rotation));

        self.pose.position += translation;
        self.pose.rotation = wrap_angle(gyro_state.yaw - self.gyro_offset);
    }

    pub fn set_current_landmark(&mut self, landmark: Landmark) {
        let landmark_pose = landmark.pose();
        let gyro_state = ROBOT.get_gyro_state();
        self.pose = landmark_pose;
        self.gyro_offset = wrap_angle(gyro_state.yaw - landmark_pose.rotation);
    }
}
