use std::f32::consts::PI;

use crate::ROBOT;
use crate::math::{MecanumVelocities, Pose, Twist, mecanum};

#[derive(Debug, Clone, Copy, Default)]
pub struct MotionState {
    pub current_twist: Twist,
    pub target_twist: Twist,
    pub current_pose: Pose,
    pub initial_yaw: f32,
}

impl MotionState {
    pub fn new() -> Self {
        let gyro_state = ROBOT.gyro_state.load();
        let initial_yaw = if gyro_state.driver_is_connected {
            gyro_state.yaw
        } else {
            f32::NAN
        };

        Self {
            initial_yaw,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let stm32_state = ROBOT.stm32_state.load();
        let gyro_state = ROBOT.gyro_state.load();

        if self.initial_yaw.is_nan() {
            self.initial_yaw = gyro_state.yaw;
        }

        let [vfl, vfr, vrl, vrr] = stm32_state
            .actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.current_pose.rotation = gyro_state.yaw - self.initial_yaw;
        if self.current_pose.rotation > PI {
            self.current_pose.rotation -= PI * 2.0;
        } else if self.current_pose.rotation < -PI {
            self.current_pose.rotation += PI * 2.0;
        }
        self.current_twist =
            Twist::from_mecanum_velocities(MecanumVelocities::new(vfl, vfr, vrl, vrr));
    }
}
