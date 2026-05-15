use std::{f32::consts::FRAC_PI_2, fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::{Action, rotate_arm::ArmRotationPreset},
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    devices::maixcam::circle::MaixcamCircleColor,
    math::{Pose, Twist},
};

#[derive(Debug, Default)]
pub struct CalibratePlacement {
    motion_policy: MotionPolicy,
    arm_rotation: ArmRotationPreset,
    mode: CalibrateMode,

    initial_rotation: f32,
}

impl CalibratePlacement {
    pub fn from_mode(mode: CalibrateMode) -> Self {
        Self {
            motion_policy: MotionPolicyPreset::CalibrationPlace.to_motion_policy(),
            mode,
            arm_rotation: ArmRotationPreset::Left,
            ..Default::default()
        }
    }

    pub fn ground() -> Self {
        Self::from_mode(CalibrateMode::Ground)
    }
    pub fn stack() -> Self {
        Self::from_mode(CalibrateMode::Stack)
    }

    pub fn with_arm_rotation(mut self, rotation: ArmRotationPreset) -> Self {
        self.arm_rotation = rotation;
        self
    }
}

impl Action for CalibratePlacement {
    fn start(&mut self) {
        ROBOT
            .get_stm32_controller()
            .set_yaw_servo(self.arm_rotation.to_angle());
        self.initial_rotation = ROBOT.odometry_state.load().pose.rotation;
    }

    fn update(&mut self, dt: Duration) {
        let current_rotation = ROBOT.odometry_state.load().pose.rotation;
        let maixcam_state = ROBOT.maixcam_state.load();
        let circle = maixcam_state.circles[0];
        let circle_position = match circle.color {
            MaixcamCircleColor::Blue =>     circle.position + Vec2::new(500.0, 0.0),
            MaixcamCircleColor::Green =>    circle.position,
            MaixcamCircleColor::Red =>      circle.position - Vec2::new(500.0, 0.0),
            MaixcamCircleColor::Unknown =>  circle.position
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

        let (linear_error, angular_error) = current_state.difference(target_state).to_components();

        // Get PID outputs from motion_policy
        let (mut linear_output, angular_output) =
            self.motion_policy.update(linear_error, angular_error, dt);

        // Rotate linear_output back to world space
        linear_output = linear_output.rotate(Vec2::from_angle(match self.arm_rotation {
            ArmRotationPreset::Left => -FRAC_PI_2,
            ArmRotationPreset::Right => FRAC_PI_2,
            _ => 0.0,
        }));

        let target_twist = Twist::new(linear_output, angular_output);
        ROBOT.get_stm32_controller().set_twist(target_twist);
    }

    fn stop(&mut self) {
        ROBOT.get_stm32_controller().set_twist(Twist::ZERO);
    }

    fn current_action(&self) -> &dyn Action {
        self
    }

    fn is_finished(&self) -> bool {
        self.motion_policy.is_settled() || !ROBOT.maixcam_state.load().driver_is_connected
    }
}

impl Display for CalibratePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Display: {:?}", self)
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
            CalibrateMode::Ground => Vec2::new(120.0, 120.0),
            CalibrateMode::Stack => Vec2::new(120.0, 120.0),
        }
    }
}
