pub mod circle;
pub mod driver;
pub mod sample;
pub mod state;
pub mod message;
use std::time::Duration;

use crate::devices::maixcam::{driver::MaixcamDriver, message::MaixcamMessage, state::MaixcamState};

const MAIXCAM_DOTENV_KEY: &str = "MAIXCAM_IP";


/// The Maixcam does nothing but send circle coordinates, so it doesn't really need to scale for the time being
/// A packet of data is formatted like this:
/// [START] [ID] [LEN] [DATA] [CHECKSUM]
/// Where pos_x/y is a float from 0 to RESOLUTION_WIDTH/HEIGHT mapped into the range of 0..10000

pub fn spawn_maixcam_thread() {
    std::thread::spawn(move || {
        let mut driver = MaixcamDriver::new();
        let mut state = MaixcamState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(20) {
                continue;
            }
            state.dt = now.duration_since(last_update);
            last_update = now;

            match driver.try_read_frame() {
                Ok(messages) => {
                    state.error = None;
                    for message in messages {
                        match message {
                            MaixcamMessage::CircleData(circles) => {
                                state.circles = circles;
                            }
                        }
                    }
                }
                Err(msg) => {
                    state.circles.clear();
                    state.driver_is_connected = false;
                    state.error = Some(msg);
                    state.publish();
                    driver.reconnect();
                }
            }
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}




