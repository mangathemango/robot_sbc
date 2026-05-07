use crate::devices::DriverPort;
use crate::ROBOT;
use std::sync::Arc;

const GYRO_DOTENV_KEY: &str = "GYRO_PATH";


pub fn spawn_gyro_thread() {
    std::thread::spawn(move || {
        let mut driver = GyroDriver::new();
        let mut state = GyroState::new();

        loop {
            match driver.try_read_frame() {
                Ok(sample) => {
                    state.error_msg = None;
                    state.update(sample);
                }
                Err(msg) => {
                    state.error_msg = Some(msg);
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            };
            state.driver_is_connected = driver.is_connected();
            ROBOT.gyro_state.store(Arc::new(state.clone()));
        }
    });
}

/// Driver struct to read + parse data sent from the gyro
#[derive(Debug)]
pub struct GyroDriver {
    port: DriverPort,
}

impl GyroDriver {
    pub fn new() -> Self {
        GyroDriver {
            port: DriverPort::from_dotenv_key(GYRO_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverPort::from_dotenv_key(GYRO_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    /// Reads serial bytes coming from self.port until a valid frame of bytes can be parsed into a GyroSample
    ///
    /// A valid frame of bytes coming from the HWT101CT has this structure:
    /// [0x55] [0x53] [...] [yaw] [...] [gy] [gz]
    pub fn try_read_frame(&mut self) -> Result<GyroSample, String> {
        match &mut self.port {
            DriverPort::Disconnected(msg) => Err(format!("Gyro driver not active: {}", msg)),
            DriverPort::Connected(port) => {
                let mut buffer = [0; 1];
                let mut frame = Vec::new();
                loop {
                    match port.read_exact(&mut buffer) {
                        Ok(_) => {}
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                            continue;
                        }
                        Err(e) => {
                            self.port = DriverPort::Disconnected(
                                format!("Gyro read frame error: {}", e).into(),
                            );
                            return Err(format!("Gyro read frame error: {}", e));
                        }
                    }

                    let byte = buffer[0];

                    // Always push
                    frame.push(byte);

                    // Keep frame from growing infinitely
                    if frame.len() > 22 {
                        frame.remove(0);
                    }

                    // Try to detect valid frame
                    if frame.len() >= 2 {
                        // Check header alignment
                        if frame[0] != 0x55 {
                            frame.remove(0);
                            continue;
                        }

                        if frame[1] != 0x53 {
                            frame.remove(0);
                            continue;
                        }
                    }

                    if frame.len() == 22 {
                        let yaw = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;
                        let gy =
                            i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 2000.0;
                        let gz =
                            i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 2000.0;

                        let sample = GyroSample { yaw, gy, gz };
                        return Ok(sample);
                    }
                }
            }
        }
    }
}

/// A data sample read from the gyroscope
#[derive(Debug, Default, Clone, Copy)]
pub struct GyroSample {
    yaw: f32,
    gy: f32,
    gz: f32,
}

/// Current gyroscope state
#[derive(Debug, Default, Clone)]
pub struct GyroState {
    /// flag to indicate activity
    pub driver_is_connected: bool,
    pub error_msg: Option<String>,

    /// The first recorded yaw for relative yaw calculation for 0 point
    pub initial_yaw: f32,
    /// Current yaw recorded from gyro
    pub current_yaw: f32,
    /// Relative yaw (with respect to initial_yaw)
    pub relative_yaw: f32,
    /// y angular acceleration
    pub gy: f32,
    /// z angular acceleration
    pub gz: f32,
}

impl GyroState {
    pub fn new() -> Self {
        GyroState {
            initial_yaw: f32::NAN,
            driver_is_connected: true,
            ..Default::default()
        }
    }

    pub fn update(&mut self, sample: GyroSample) {
        self.current_yaw = sample.yaw;
        self.gy = sample.gy;
        self.gz = sample.gz;
        if self.initial_yaw.is_nan() {
            self.initial_yaw = sample.yaw
        }
        self.relative_yaw = self.current_yaw - self.initial_yaw;
        if self.relative_yaw > 180.0 {
            self.relative_yaw -= 360.0;
        }
        if self.relative_yaw < -180.0 {
            self.relative_yaw += 360.0;
        }
    }
}


