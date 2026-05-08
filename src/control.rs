pub mod kinematic;

use std::f32::consts::PI;
use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::Vec2;
use crate::ROBOT;
use crate::math::{PidController, Pose, Twist, wrap_angle};

pub fn spawn_main_controller_thread() {
    std::thread::spawn(|| {
        let linear_pid = PidController::new(3.0, 0.0, 0.0, 0.01, 1.0);
        let angular_pid = PidController::new(0.001, 0.0, 0.0, 0.01, 1.0);
        let mut controller_state = ControllerState::new(linear_pid, angular_pid);
        controller_state.publish();
        loop {
            controller_state.move_to(Pose {
                position: Vec2 { x: 0.1, y: 0.1 },
                rotation: 0.0,
            });
            controller_state.move_to(Pose {
                position: Vec2 { x: -0.1, y: 0.1 },
                rotation: 0.0,
            });
            controller_state.move_to(Pose {
                position: Vec2 { x: -0.1, y: -0.1 },
                rotation: 0.0,
            });
            controller_state.move_to(Pose {
                position: Vec2 { x: 0.1, y: -0.1 },
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

        let kinematic_state = ROBOT.kinematic_state.load();

        let error_pose = kinematic_state.current_pose.difference(self.target_pose);
        let linear_error = error_pose.position.length();
        let angular_error = error_pose.rotation;

        let linear_correction_speed = self.linear_pid.update(linear_error, dt);
        let linear_correction_vec = Vec2::from_angle(wrap_angle(kinematic_state.current_pose.rotation + PI)).rotate(
            error_pose.position.normalize_or_zero() * linear_correction_speed
        );
        let angular_correction = self.angular_pid.update(angular_error, dt);

        self.target_twist = Twist::new(linear_correction_vec, angular_correction);
        stm32_controller.set_twist(self.target_twist);
        self.publish();
    }

    pub fn move_to(&mut self, pose: Pose) {
        self.target_pose = pose;
        let mut settled_frames = 0;
        let mut last = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now.duration_since(last);
            self.dt = dt;
            self.update(dt);
            if self.linear_pid.is_settled()  {
                settled_frames += 1;
            } else {
                settled_frames = 0;
            }
            if settled_frames >= 10 {
                self.stop();
                break;
            }
            last = now;
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn publish(&self) {
        ROBOT.controller_state.store(Arc::new(*self));
    }

    pub fn stop(&mut self) {}
}
