use std::{
    f32::consts::{FRAC_PI_2, PI},
    fmt::Display,
    time::Duration,
};

use glam::Vec2;

use crate::{
    ROBOT,
    control::{
        actions::Action,
        motion::{MotionPolicy, MotionPolicyPreset},
    },
    devices::maixcam::circle::MaixcamCircleColor,
    math::{Pose, Twist},
};

// This command assumes the arm is on the right side by hard code bc we only do that lmoa

pub struct CalibrateSource {
    // Configs
    move_time: Duration,
    circle_stable_time: Duration,
    circle_stable_speed: f32,

    // States
    state: CalibrateState,
    motion_policy: MotionPolicy,
    timer: Duration,
    initial_rotation: f32,
}

impl CalibrateSource {
    pub fn new() -> Self {
        Self {
            move_time: Duration::from_millis(3500),
            circle_stable_time: Duration::from_millis(500),
            circle_stable_speed: 0.1,

            state: CalibrateState::WaitingForUnstable,
            timer: Duration::ZERO,
            motion_policy: MotionPolicyPreset::CalibrationSource.to_motion_policy(),

            initial_rotation: 0.0,
        }
    }
}

impl Action for CalibrateSource {
    fn start(&mut self) {
        self.initial_rotation = 0.0;
    }

    fn update(&mut self, dt: Duration) {
        let circle = ROBOT.maixcam_state().find_priority_ring(&[
            MaixcamCircleColor::Red,
            MaixcamCircleColor::Green,
            MaixcamCircleColor::Blue,
        ]);
        match self.state {
            CalibrateState::WaitingForUnstable => {
                if let Some(circle) = circle {
                    if circle.speed > self.circle_stable_speed {
                        self.timer += dt;
                        if self.timer > self.circle_stable_time {
                            self.state = CalibrateState::WaitingForStable;
                        }
                    } else {
                        self.timer = Duration::ZERO;
                    }
                }
            }

            CalibrateState::WaitingForStable => {
                if let Some(circle) = circle {
                    if circle.speed < self.circle_stable_speed {
                        self.timer += dt;
                        if self.timer > self.circle_stable_time {
                            self.state = CalibrateState::MovingToTarget;
                        }
                    } else {
                        self.timer = Duration::ZERO;
                    }
                }
            }
            CalibrateState::MovingToTarget => {
                self.timer += dt;

                if self.timer > self.move_time {
                    ROBOT.stm32_controller().set_twist(Twist::ZERO);
                    self.state = CalibrateState::WaitingForStable;
                    return;
                }
                if let Some(circle) = circle {
                    let current_rotation = ROBOT.odometry_state().pose.rotation;
                    // Move the robot linearly so that the circle ends up in the target position while keeping the initial rotation stable
                    let current_state = Pose {
                        position: circle.position,
                        rotation: current_rotation,
                    };

                    let target_state = Pose {
                        position: Vec2::new(0.5, 0.5),
                        rotation: self.initial_rotation,
                    };

                    let (linear_error, angular_error) =
                        current_state.difference(target_state).to_components();

                    // Get PID outputs from motion_policy
                    let (mut linear_output, angular_output) =
                        self.motion_policy.update(linear_error, angular_error, dt);
                    linear_output.y *= -1.0;
                    linear_output = linear_output.rotate(Vec2::from_angle(FRAC_PI_2));
                    let target_twist = Twist::new(linear_output, angular_output);
                    ROBOT.stm32_controller().set_twist(target_twist);
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.state == CalibrateState::MovingToTarget && self.motion_policy.is_settled()
    }
}

impl Display for CalibrateSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:?}: {:?}/{:?}\n\nLinear PID: {}\n\nAngular PID: {}\n\nSettle time: {}ms",
            self.state,
            self.timer,
            match self.state {
                CalibrateState::WaitingForStable | CalibrateState::WaitingForUnstable =>
                    self.circle_stable_time,
                CalibrateState::MovingToTarget => self.move_time,
            },
            self.motion_policy.linear_pid,
            self.motion_policy.angular_pid,
            self.motion_policy.settle_time.as_secs_f32() * 1000.0
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalibrateState {
    WaitingForUnstable,
    WaitingForStable,
    MovingToTarget,
}
