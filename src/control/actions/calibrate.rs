use std::{fmt::Display, time::Duration};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::{Action, rotate_arm::ArmRotationPreset},
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    devices::maixcam::color::MaixcamCircleColor,
    math::{Pose, Twist},
};

#[derive(Debug, Default)]
pub struct Calibrate {
    motion_policy: MotionPolicy,
    arm_rotation: ArmRotationPreset,
    mode: CalibrateMode,

    elapsed: Duration,
    initial_pose: Pose,
}

impl Calibrate {
    pub fn from_mode(mode: CalibrateMode) -> Self {
        Self {
            motion_policy: mode.motion_policy(),
            mode,
            arm_rotation: ArmRotationPreset::Left,
            ..Default::default()
        }
    }

    pub fn at_place_zone_ground() -> Self {
        Self::from_mode(CalibrateMode::PlaceZoneGround)
    }
    pub fn at_place_zone_stack() -> Self {
        Self::from_mode(CalibrateMode::PlaceZoneStack)
    }
    pub fn at_source_zone() -> Self {
        Self::from_mode(CalibrateMode::SourceZone {
            stable_time: Duration::from_millis(500),
            calibrate_time: Duration::from_millis(2000),
        })
    }

    pub fn with_arm_rotation(mut self, rotation: ArmRotationPreset) -> Self {
        self.arm_rotation = rotation;
        self
    }

    pub fn move_to_target(&mut self, dt: Duration) {
        let current_pose = ROBOT.odometry_state.load().pose;
        let current_circle_position = ROBOT.maixcam_state.load().circle_position;

        // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
        let current_state = Pose {
            position: current_circle_position,
            rotation: current_pose.rotation,
        };

        let target_state = Pose {
            position: self.mode.target_circle_position(),
            rotation: self.initial_pose.rotation,
        };

        let (linear_error, angular_error) = current_state.difference(target_state).to_components();

        // Get PID outputs from motion_policy
        let (mut linear_output, angular_output) =
            self.motion_policy.update(linear_error, angular_error, dt);

        // Rotate linear_output back to world space
        linear_output = linear_output.rotate(Vec2::from_angle(-current_pose.rotation));

        let target_twist = Twist::new(linear_output, angular_output);
        ROBOT.get_stm32_controller().set_twist(target_twist);
    }
}

impl Action for Calibrate {
    fn start(&mut self) {
        ROBOT
            .get_stm32_controller()
            .set_yaw_servo(self.arm_rotation.to_angle());
        self.initial_pose = ROBOT.odometry_state.load().pose;
    }

    fn update(&mut self, dt: Duration) {
        todo!("Finish Calibrate implementation");
    }

    fn stop(&mut self) {}

    fn current_action(&self) -> &dyn Action {
        self
    }

    fn is_finished(&self) -> bool {
        true
    }
}

impl Display for Calibrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Display: {:?}", self)
    }
}

#[derive(Debug, Default)]
pub enum CalibrateMode {
    #[default]
    PlaceZoneGround,
    SourceZone {
        stable_time: Duration,
        calibrate_time: Duration,
    },
    PlaceZoneStack,
}

impl CalibrateMode {
    pub fn target_circle_position(&self) -> Vec2 {
        match self {
            CalibrateMode::PlaceZoneGround => Vec2::new(120.0, 120.0),
            CalibrateMode::PlaceZoneStack => Vec2::new(120.0, 120.0),
            CalibrateMode::SourceZone { .. } => Vec2::new(120.0, 120.0),
        }
    }

    pub fn motion_policy(&self) -> MotionPolicy {
        match self {
            CalibrateMode::SourceZone { .. } => MotionPolicyPreset::CalibrationSource,
            _ => MotionPolicyPreset::CalibrationPlace,
        }
        .to_motion_policy()
    }
}
