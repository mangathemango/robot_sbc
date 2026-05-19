use std::{fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::Action,
        landmark::Landmark,
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    math::{Pose, Twist},
};

#[derive(Debug, Clone, Default)]
pub struct Move {
    target_pose: Pose,
    motion_policy: MotionPolicy,
    policy_preset: MotionPolicyPreset,
    mode: ControlMode,
}

impl Move {
    pub fn to_pose(pose: Pose) -> Self {
        Self {
            target_pose: pose,
            ..Default::default()
        }
    }

    pub fn to(landmark: Landmark) -> Self {
        Self {
            target_pose: landmark.pose(),
            ..Default::default()
        }
    }

    pub fn policy(mut self, preset: MotionPolicyPreset) -> Self {
        self.policy_preset = preset;
        self.motion_policy = preset.to_motion_policy();
        self
    }

    pub fn mode(mut self, mode: ControlMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Action for Move {
    fn start(&mut self) {
        let current_pose = ROBOT.odometry_state().pose;
        match self.mode {
            ControlMode::Full => (),
            ControlMode::RotateOnly => {
                self.target_pose.position = current_pose.position;
            }
            ControlMode::TranslateOnly => {
                self.target_pose.rotation = current_pose.rotation;
            }
        }
    }

    fn update(&mut self, dt: Duration) {
        let current_pose = ROBOT.odometry_state().pose;

        let (linear_error, angular_error) =
            current_pose.difference(self.target_pose).to_components();

        // Get PID outputs from motion_policy
        let (mut linear_output, angular_output) =
            self.motion_policy.update(linear_error, angular_error, dt);

        // Rotate linear_output back to world space
        linear_output = linear_output.rotate(Vec2::from_angle(-current_pose.rotation));

        let target_twist = Twist::new(linear_output, angular_output);
        ROBOT.stm32_controller().set_twist(target_twist);
    }

    fn is_finished(&self) -> bool {
        self.motion_policy.is_settled()
    }

    fn stop(&mut self) {
        let stm32_controller = ROBOT.stm32_controller();
        stm32_controller.set_wheel_velocities([0, 0, 0, 0]);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Moving to {} with policy {:?} and mode {:?}\n\nLinear PID: {}\n\nAngular PID: {}\n\nSettle time: {}ms",
            self.target_pose,
            self.policy_preset,
            self.mode,
            self.motion_policy.linear_pid,
            self.motion_policy.angular_pid,
            self.motion_policy.settle_time.as_millis()
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ControlMode {
    #[default]
    Full,
    TranslateOnly,
    RotateOnly,
}
