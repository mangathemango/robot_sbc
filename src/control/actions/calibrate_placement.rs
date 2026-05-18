use std::{f32::consts::FRAC_PI_2, fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::{Action, rotate_arm::ArmRotationPreset},
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    devices::maixcam::circle::{MaixcamCircle, MaixcamCircleColor},
    math::{Pose, Twist},
};

#[derive(Debug, Default)]
pub struct CalibratePlacement {
    motion_policy: MotionPolicy,
    chosen_circle: Option<MaixcamCircle>,
    mode: CalibrateMode,
    initial_rotation: f32,
}

impl CalibratePlacement {
    pub fn from_mode(mode: CalibrateMode) -> Self {
        Self {
            motion_policy: MotionPolicyPreset::CalibrationPlace.to_motion_policy(),
            mode,
            ..Default::default()
        }
    }

    pub fn ground() -> Self {
        Self::from_mode(CalibrateMode::Ground)
    }
    pub fn stack() -> Self {
        Self::from_mode(CalibrateMode::Stack)
    }

    pub fn while_keeping_rotation(mut self, rotation: f32) -> Self {
        self.initial_rotation = rotation;
        self
    }
}

impl Action for CalibratePlacement {
    fn start(&mut self) {
        ROBOT
            .get_stm32_controller()
            .set_yaw_servo(ArmRotationPreset::Calibration.to_angle());
    }

    fn update(&mut self, dt: Duration) {
        let current_rotation = ROBOT.get_odometry_state().pose.rotation;
        let maixcam_state = ROBOT.get_maixcam_state();
        self.chosen_circle = maixcam_state.find_priority_ring(&[
            MaixcamCircleColor::Green,
            MaixcamCircleColor::Blue,
            MaixcamCircleColor::Red,
        ]);

        if let Some(circle) = self.chosen_circle {
            let circle_position = match circle.color {
                MaixcamCircleColor::Blue => circle.position + Vec2::new(0.5, 0.0),
                MaixcamCircleColor::Green => circle.position,
                MaixcamCircleColor::Red => circle.position + Vec2::new(-0.5, 0.0),
            };

            // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
            let current_state = Pose {
                position: circle_position,
                rotation: current_rotation,
            };

            let target_state = Pose {
                position: self.mode.target_circle_position(),
                rotation: self.initial_rotation,
            };

            let (linear_error, angular_error) =
                current_state.difference(target_state).to_components();

            // Get PID outputs from motion_policy
            let (mut linear_output, angular_output) =
                self.motion_policy.update(linear_error, angular_error, dt);
            linear_output = linear_output.rotate(Vec2::from_angle(-FRAC_PI_2));

            let target_twist = Twist::new(linear_output, angular_output);
            ROBOT.get_stm32_controller().set_twist(target_twist);
        }
    }

    fn stop(&mut self) {
        ROBOT.get_stm32_controller().set_twist(Twist::ZERO);
    }

    fn is_finished(&self) -> bool {
        self.motion_policy.is_settled() || !ROBOT.get_maixcam_state().driver_is_connected
    }
}

impl Display for CalibratePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Calibrating placment with mode {:?}\nChosen Circle: {}\nTarget position: {}\nMotion Policy: {:?}", 
            self.mode, self.chosen_circle.unwrap_or_default(), self.mode.target_circle_position(), self.motion_policy)
    }
}

#[derive(Debug, Default)]
pub enum CalibrateMode {
    #[default]
    Ground,
    Stack,
}

impl CalibrateMode {
    pub fn target_circle_position(&self) -> Vec2 {
        match self {
            CalibrateMode::Ground => Vec2::new(0.5, 0.5),
            CalibrateMode::Stack => Vec2::new(0.5, 0.5),
        }
    }
}
