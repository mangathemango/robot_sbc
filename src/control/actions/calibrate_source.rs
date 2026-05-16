use std::{fmt::Display, time::Duration};

use glam::Vec2;

use crate::{ROBOT, control::{actions::Action, motion::{MotionPolicy, MotionPolicyPreset}}};

// This command assumes the arm is on the right side by hard code bc we only do that lmoa


pub struct CalibrateSource {
    // Configs
    move_time: Duration,
    circle_stable_time: Duration,
    circle_stable_speed: f32,

    // States
    state: CalibrateState,
    circle_speed: f32,
    motion_policy: MotionPolicy,
    timer: Duration,
    last_circle_position: Vec2,
    initial_rotation: f32,
}

impl CalibrateSource {
    pub fn new() -> Self {
        Self {
            move_time: Duration::from_millis(3500),
            circle_stable_time: Duration::from_millis(500),
            circle_stable_speed: 3.0,

            state: CalibrateState::WaitingForStable,
            circle_speed: 0.0,
            timer: Duration::ZERO,
            motion_policy: MotionPolicyPreset::CalibrationSource.to_motion_policy(),

            last_circle_position: Vec2::NAN,
            initial_rotation: 0.0,
        }
    }
}

impl Action for CalibrateSource {
    fn start(&mut self) {
        self.initial_rotation = 0.0;
    }

    fn update(&mut self, dt: Duration) {
        if ROBOT.get_maixcam_state().circles.is_empty() {
            return;
        }
        let current_circle_position = ROBOT.get_maixcam_state().circles[0].position;
        let circle_velocity = current_circle_position.distance(self.last_circle_position) / dt.as_secs_f32();

        match self.state {
            CalibrateState::WaitingForStable => {

            }
            CalibrateState::MovingToTarget =>  {

            },
        }
    }

    fn current_action(&self) -> &dyn Action {
        self
    }

    fn is_finished(&self) -> bool {
        true
    }
}

impl Display for CalibrateSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Calibrate source")
    }
}


#[derive(Debug, Clone, Copy)]
enum CalibrateState {
    WaitingForStable,
    MovingToTarget
}