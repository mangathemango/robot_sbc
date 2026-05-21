use std::time::Duration;

use glam::Vec2;

use crate::math::PidController;

#[derive(Debug, Clone, Copy)]
pub struct MotionPolicy {
    pub linear_pid: PidController,
    pub angular_pid: PidController,
    pub settle_time: Duration,
}

impl MotionPolicy {

    pub fn update(&mut self, linear_error: Vec2, angular_error: f32, dt: Duration) -> (Vec2, f32) {
        let linear_direction = linear_error.normalize_or_zero();
        let linear_correction_speed = self.linear_pid.update(linear_error.length(), dt);
        let linear_correction = linear_direction * linear_correction_speed;

        let angular_correction = self.angular_pid.update(angular_error, dt);

        (linear_correction, angular_correction)
    }

    pub fn is_settled(&self) -> bool {
        self.linear_pid.is_settled_for(self.settle_time)
            && self.angular_pid.is_settled_for(self.settle_time)
    }
}

impl Default for MotionPolicy {
    fn default() -> Self {
        MotionPolicyPreset::default().to_motion_policy()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum MotionPolicyPreset {
    #[default]
    Precise,
    Aggressive,
    CalibrationSource,
    CalibrationPlace,
    Custom(MotionPolicy),
}

impl MotionPolicyPreset {
    pub fn to_motion_policy(&self) -> MotionPolicy {
        match self {
            Self::Precise => MotionPolicy {
                linear_pid: PidController::new(0.05, 0.0, 0.0, 0.001, 0.0),
                angular_pid: PidController::new(-0.008, -0.0005, -0.0005, 0.1, 1.0),
                settle_time: Duration::from_millis(1000), 
            },
            Self::Aggressive => MotionPolicy {
                linear_pid: PidController::new(5.0, 0.0, 1.0, 0.05, 1.0),
                angular_pid: PidController::new(-0.01, 0.0, -0.005, 0.04, 0.0),
                settle_time: Duration::from_millis(200),
            },
            Self::CalibrationSource => MotionPolicy {
                linear_pid: PidController::new(0.1, 0.0, 0.1, 0.2, 1.0),
                angular_pid: PidController::new(-0.01, 0.0, -0.005, 0.04, 0.0),
                settle_time: Duration::from_millis(1000),
            },
            Self::CalibrationPlace => MotionPolicy {
                linear_pid: PidController::new(0.1, 0.0, 0.1, 0.05, 1.0),
                angular_pid: PidController::new(-0.01, 0.0, -0.005, 0.001, 0.0),
                settle_time: Duration::from_millis(1000),   
            },
            Self::Custom(profile) => *profile,
        }
    }
}
