pub mod motion;
use std::{sync::Arc, time::Duration};

use crate::{ROBOT, control::motion::MotionState};

pub fn spawn_control_thread() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(100));
        let mut motion_state = MotionState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            motion_state.dt = now.duration_since(last_update);
            last_update = now;

            motion_state.update();
            ROBOT.motion_state.store(Arc::new(motion_state));
        }
    });
}