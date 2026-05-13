use crate::devices::maixcam::{driver::MaixcamDriver, state::MaixcamState};

pub mod color;
pub mod driver;
pub mod sample;
pub mod state;

const MAIXCAM_DOTENV_KEY: &str = "MAIXCAM_PATH";
const MAIXCAM_START_BYTE: u8 = 0x69;
const MAIXCAM_CAMERA_RESOLUTION_WIDTH: f32 = 360.0;
const MAIXCAM_CAMERA_RESOLUTION_HEIGHT: f32 = 240.0;


/// The Maixcam does nothing but send circle coordinates, so it doesn't really need to scale for the time being
/// A packet of data is formatted like this:
/// [Start] [pos_x] [pos_x] [pos_y] [pos_y]
/// Where pos_x/y is a float from 0 to RESOLUTION_WIDTH/HEIGHT mapped into the range of 0..10000

pub fn spawn_maixcam_thread() {
    std::thread::spawn(move || {
        let mut driver = MaixcamDriver::new();
        let mut state = MaixcamState::new();
        let mut last_update = std::time::Instant::now();

        loop {
            let now = std::time::Instant::now();
            state.dt = now.duration_since(last_update);
            last_update = now;

            match driver.try_read_frame() {
                Ok(sample) => {
                    state.update(sample);
                }
                Err(_) => {
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}




