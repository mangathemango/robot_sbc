pub mod motion;
use std::{sync::Arc, time::Duration};

use crate::{ROBOT, control::motion::MotionState};

pub fn spawn_control_thread() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(100));
        let mut motion_state = MotionState::new();

        loop {
            motion_state.update();
            ROBOT.motion_state.store(Arc::new(motion_state));
        }
    });
}