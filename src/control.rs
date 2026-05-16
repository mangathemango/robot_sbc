pub mod actions;
pub mod landmark;
pub mod motion;
pub mod sequences;
pub mod states;

use std::fmt::Debug;
use std::time::{Duration, Instant};

use crate::control::actions::Action;
use crate::control::sequences::Sequence;
use crate::control::sequences::main::main_sequence;
use crate::ROBOT;

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

#[derive(Default)]
pub struct Controller {
    pub sequence: Sequence,
    pub state: ControllerState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            sequence: Sequence::new("Controller"),
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.state.current_command_debug_string = format!("{}", self.sequence);
        self.sequence.update(dt);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ControllerState {
    pub current_command_debug_string: String,
    pub dt: Duration,
}

impl ControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&self) {
        ROBOT.set_controller_state(self.clone());
    }
}
