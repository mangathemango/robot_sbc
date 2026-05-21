use std::time::{Duration, Instant};

use crate::ROBOT;
use crate::control::actions::Action;
use crate::control::routines::main::main_sequence;
use crate::control::routines::test::test_sequence;

pub fn spawn_action_executor_thread() {
    std::thread::spawn(|| {
        ROBOT.action_queue_mut().enqueue(test_sequence());
        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(20) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }

            let mut action_queue = ROBOT.action_queue_mut();
            if ROBOT.stm32_state().key1_is_pressed() {
                if action_queue.is_finished() {
                    action_queue.enqueue(test_sequence());
                } else {
                    action_queue.abort();
                }
            }
            action_queue.update(dt);
            last_tick = now;
        }
    });
}
