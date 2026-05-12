use std::{sync::OnceLock, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        ControllerState,
        actions::Action,
        landmark::{self, Landmark},
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    devices::stm32::Stm32Controller,
    math::{PidController, Pose, Twist},
};

#[derive(Debug, Clone, Default)]
pub struct Move {
    target_pose: Pose,
    policy: MotionPolicy,
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
        self.policy = preset.to_motion_policy();
        self
    }

    pub fn mode(mut self, mode: ControlMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Action for Move {
    fn start(&mut self, state: &mut ControllerState) {
        let current_pose = ROBOT.odometry_state.load().pose;
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

    fn update(&mut self, state: &mut ControllerState, dt: Duration) {
        let stm32_controller = ROBOT.get_stm32_controller();
        let current_pose = ROBOT.odometry_state.load().pose;

        let (linear_error, angular_error) =
            current_pose.difference(self.target_pose).to_components();
        let (mut linear_output, angular_output) =
            self.policy.update(linear_error, angular_error, dt);
        linear_output = linear_output.rotate(Vec2::from_angle(-current_pose.rotation));

        let target_twist = Twist::new(linear_output, angular_output);
        stm32_controller.set_twist(target_twist);

        state.target_pose = self.target_pose;
    }

    fn is_finished(&self) -> bool {
        self.policy.is_settled()
    }

    fn stop(&mut self, state: &mut ControllerState) {
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_wheel_velocities([0, 0, 0, 0]);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ControlMode {
    #[default]
    Full,
    TranslateOnly,
    RotateOnly,
}
