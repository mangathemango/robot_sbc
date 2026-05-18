use std::fmt::Debug;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crate::ROBOT;
use crate::control::actions::Action;
use crate::control::actions::general::Sequence;
use crate::control::routines::main::main_sequence;
use crate::control::routines::test::test_sequence;

pub fn spawn_scheduler_thread() {
    std::thread::spawn(|| {
        let mut scheduler = Scheduler::new();
        scheduler.sequence.enqueue(test_sequence());
        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(20) {
                continue;
            }
            if ROBOT
                .get_stm32_state()
                .start_flag
                .swap(false, Ordering::Relaxed)
            {
                scheduler.sequence.enqueue(main_sequence());
            }
            scheduler.update(dt);
            scheduler.state.publish();
            last_tick = now;
        }
    });
}

#[derive(Default)]
pub struct Scheduler {
    pub sequence: Sequence,
    pub state: SchedulerState,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            sequence: Sequence::new("Controller"),
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.state.current_command_debug_string = format!("{}", self.sequence);
        self.state.dt = dt;
        self.sequence.update(dt);
    }
}

#[derive(Debug, Default, Clone)]
pub struct SchedulerState {
    pub current_command_debug_string: String,
    pub dt: Duration,
}

impl SchedulerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&self) {
        ROBOT.set_controller_state(self.clone());
    }
}
