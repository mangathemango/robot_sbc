use crate::devices::DriverPort;
use glam::Vec2;
use crate::ROBOT;
use std::sync::Arc;

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
            ROBOT.maixcam_state.store(Arc::new(state.clone()));
        }
    });
}

#[derive(Debug)]
pub struct MaixcamDriver {
    port: DriverPort,
}

impl MaixcamDriver {
    pub fn new() -> Self {
        MaixcamDriver {
            port: DriverPort::from_dotenv_key(MAIXCAM_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverPort::from_dotenv_key(MAIXCAM_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn try_read_frame(&mut self) -> Result<MaixcamSample, String> {
        match &mut self.port {
            DriverPort::Disconnected(msg) => Err(format!("Maixcam driver not active: {}", msg)),
            DriverPort::Connected(port) => {
                let mut buffer = [0; 1];
                let mut frame = Vec::<u8>::new();
                let mut idx = 0;
                loop {
                    match port.read(&mut buffer) {
                        Ok(_) => {}
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::TimedOut {
                                continue;
                            } else {
                                self.port = DriverPort::Disconnected(
                                    format!("Maixcam read frame error: {}", e).into(),
                                );
                                return Err(format!("Read from Maixcam failed: {}", e));
                            }
                        }
                    }

                    let byte = buffer[0];
                    if idx == 0 && byte != MAIXCAM_START_BYTE {
                        continue;
                    }
                    frame.push(byte);
                    idx += 1;
                    if idx == 6 {
                        return Ok(MaixcamSample {
                            circle_position_x: u16::from_le_bytes([frame[1], frame[2]]),
                            circle_position_y: u16::from_le_bytes([frame[3], frame[4]]),
                            circle_color_id: frame[5],
                        });
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MaixcamCircleColor {
    Unknown,
    Red,
    Green,
    Blue,
}

impl MaixcamCircleColor {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => MaixcamCircleColor::Red,
            2 => MaixcamCircleColor::Green,
            3 => MaixcamCircleColor::Blue,
            _ => MaixcamCircleColor::Unknown,
        }
    }
}
impl Default for MaixcamCircleColor {
    fn default() -> Self {
        MaixcamCircleColor::Unknown
    }
}

#[derive(Debug, Default)]
pub struct MaixcamSample {
    circle_position_x: u16,
    circle_position_y: u16,
    circle_color_id: u8,
}

#[derive(Debug, Default, Clone)]
pub struct MaixcamState {
    pub driver_is_connected: bool,
    pub circle_position: Vec2,
    pub circle_color: MaixcamCircleColor,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl MaixcamState {
    pub fn new() -> Self {
        MaixcamState::default()
    }

    pub fn update(&mut self, sample: MaixcamSample) {
        self.circle_position = Vec2 {
            x: sample.circle_position_x as f32 / 10000.0 * MAIXCAM_CAMERA_RESOLUTION_WIDTH,
            y: sample.circle_position_y as f32 / 10000.0 * MAIXCAM_CAMERA_RESOLUTION_HEIGHT,
        };
        self.circle_color = MaixcamCircleColor::from_id(sample.circle_color_id);
    }
}
