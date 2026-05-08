use std::f32::consts::PI;

use glam::Vec2;

use crate::ROBOT;
use crate::math::{MecanumVelocities, Pose, Twist, wrap_angle};
use std::{sync::Arc, time::Duration};

pub fn spawn_odometry_thread() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(100));
        let mut kinematic_state = OdometryState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            kinematic_state.dt = now.duration_since(last_update);
            last_update = now;

            kinematic_state.update(kinematic_state.dt);
            ROBOT.odometry_state.store(Arc::new(kinematic_state));
            std::thread::sleep(Duration::from_millis(10));
        }
    });
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OdometryState {
    pub current_twist: Twist,
    pub current_pose: Pose,
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
        let stm32_state = ROBOT.stm32_state.load();
        let gyro_state = ROBOT.gyro_state.load();

        if self.initial_rotation.is_nan() {
            self.initial_rotation = gyro_state.yaw;
        }

        let [vfl, vfr, vrl, vrr] = stm32_state
            .actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.current_twist = MecanumVelocities::new(vfl, vfr, vrl, vrr).to_twist();
        let translation = (self.current_twist.linear * dt.as_secs_f32())
            .rotate(Vec2::from_angle(self.current_pose.rotation));

        self.current_pose.position += translation;
        self.current_pose.rotation = wrap_angle(gyro_state.yaw - self.initial_rotation);
    }
}
