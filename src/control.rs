pub mod odometry;

use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use rand::distr::uniform::SampleRange;
use rand::prelude::*;

use crate::ROBOT;
use crate::math::{PidController, Pose, Twist, wrap_angle};
use glam::Vec2;

pub fn spawn_main_controller_thread() {
    std::thread::spawn(|| {
        let linear_pid = PidController::new(3.0, 0.0, 0.3, 0.01, 1.0);
        let angular_pid = PidController::new(0.001, 0.0, 0.0, 0.01, 1.0);
        let mut controller_state = ControllerState::new(linear_pid, angular_pid);
        // Get an RNG:
        let mut rng = rand::rng();
        controller_state.publish();
        loop {
            controller_state.move_to(Pose {
                position: Vec2 { x: rng.random_range(-0.3..0.3), y: rng.random_range(-0.3..0.3) },
                rotation: 0.0,
            });
        }
    });
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ControllerState {
    pub target_pose: Pose,
    pub target_twist: Twist,

    pub linear_pid: PidController,
    pub angular_pid: PidController,

    pub dt: Duration,
}

impl ControllerState {
    pub fn new(linear_pid: PidController, angular_pid: PidController) -> Self {
        Self {
            linear_pid,
            angular_pid,
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let stm32_controller = ROBOT.get_stm32_controller();

        let kinematic_state = ROBOT.odometry_state.load();

        let error_pose = kinematic_state.current_pose.difference(self.target_pose);
        let linear_error = error_pose.position.length();
        let angular_error = error_pose.rotation;

        let linear_correction_speed = self.linear_pid.update(linear_error, dt);
        let linear_correction_vec = (error_pose.position.normalize_or_zero()
            * linear_correction_speed)
            .rotate(Vec2::from_angle(-kinematic_state.current_pose.rotation));
        let angular_correction = self.angular_pid.update(angular_error, dt);

        self.target_twist = Twist::new(linear_correction_vec, angular_correction);

        stm32_controller.set_twist(self.target_twist);
        self.publish();
    }

    pub fn move_to(&mut self, pose: Pose) {
        self.target_pose = pose;
        let mut last = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last;
            if dt.as_secs_f32() < 0.01 {
                thread::sleep(Duration::from_millis(1));
                continue;
            }

            self.dt = dt;
            self.update(dt);
            if self.linear_pid.is_settled_for(Duration::from_millis(100)) {
                self.stop();
                break;
            }
            last = now;
            thread::sleep(Duration::from_millis(5));
        }
    }

    pub fn publish(&self) {
        ROBOT.controller_state.store(Arc::new(*self));
    }

    pub fn stop(&mut self) {
        let stm32_controller = ROBOT.get_stm32_controller();
        self.target_twist = Twist::ZERO;
        stm32_controller.set_twist(Twist::ZERO);
    }
}
