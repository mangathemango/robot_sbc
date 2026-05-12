pub mod actions;
pub mod landmark;
pub mod motion;
pub mod sequences;
pub mod states;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::control::actions::Action;
use crate::control::sequences::Sequence;
use crate::control::sequences::main::main_sequence;
use crate::math::Pose;
use crate::{ROBOT, main};

pub fn spawn_main_controller_thread() {
    std::thread::spawn(|| {
        let mut controller = Controller::new();
        controller.sequence.enqueue(main_sequence());

        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(20) {
                continue;
            }
            controller.state.dt = dt;

            controller.update(dt);

            controller.state.publish();
            last_tick = now;
        }
    });
}

#[derive(Debug, Default)]
pub struct Controller {
    pub sequence: Sequence,
    pub state: ControllerState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.state.current_command_debug_string = format!("{:?}", self.sequence.current_action());
        self.sequence.update(dt);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ControllerState {
    pub current_command_debug_string: String,
    pub target_pose: Pose,
    pub dt: Duration,
}

impl ControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&self) {
        ROBOT.controller_state.store(Arc::new(self.clone()));
    }
}
