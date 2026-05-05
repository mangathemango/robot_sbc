use crate::devices::DriverPort;
/// Driver struct to read + parse data sent from the gyro
#[derive(Debug)]
pub struct GyroDriver {
    port: DriverPort,
}

/// A data sample read from the gyroscope
#[derive(Debug, Default, Clone, Copy)]
pub struct GyroSample {
    yaw: f32,
    gy: f32,
    gz: f32,
}

/// Current gyroscope state
#[derive(Debug, Default, Clone, Copy)]
pub struct GyroState {
    /// The first recorded yaw for relative yaw calculation for 0 point
    initial_yaw: f32,
    /// Current yaw recorded from gyro
    current_yaw: f32,
    /// Relative yaw (with respect to initial_yaw)
    relative_yaw: f32,
    /// y angular acceleration
    gy: f32,
    /// z angular acceleration
    gz: f32,
    /// flag to indicate activity
    active: bool
}

impl GyroDriver {
    pub fn new() -> Self {
        GyroDriver {
            port: DriverPort::from_dotenv_key("GYRO_PORT"),
        }
    }



    fn parse_frame(frame: &[u8]) -> GyroSample {
        let yaw = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;
        let gy = i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 2000.0;
        let gz = i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 2000.0;

        GyroSample { yaw, gy, gz }
    }

    /// Reads serial bytes coming from self.port until a valid frame of bytes can be parsed into a GyroSample
    ///
    /// A valid frame of bytes coming from the HWT101CT has this structure:
    /// [0x55] [0x53] [...] [yaw] [...] [gy] [gz]
    pub fn get_sample(&mut self) -> Result<GyroSample, String> {
        match &mut self.port {
            DriverPort::Inactive => {
                self.port = DriverPort::from_dotenv_key("GYRO_PORT");
                Err("Gyro driver not active, cannot get sample\t Attempting to reconnect...".into())
            },
            DriverPort::Active(port) => {
                let mut buffer = [0; 1];
                let mut frame = Vec::new();
                loop {
                    match port.read_exact(&mut buffer) {
                        Ok(_) => {}
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                            continue;
                        }
                        Err(e) => {
                            self.port = DriverPort::Inactive;
                            return Err(format!("Gyro get_sample error: {}\nAttempting to reconnect...", e));
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
                        return Ok(Self::parse_frame(&frame));
                    }
                }
            }
        }
    }

    pub fn update_state(&mut self, state: &mut GyroState) -> Result<(), String> {
        let sample = match self.get_sample() {
            Ok(sample) => {
                state.active = true;
                sample
            },
            Err(e) => {
                state.active = false;
                return Err(e);
            } 
        };
        state.current_yaw = sample.yaw;
        state.gy = sample.gy;
        state.gz = sample.gz;
        if state.initial_yaw.is_nan() {
            state.initial_yaw = sample.yaw
        }
        state.relative_yaw = state.current_yaw - state.initial_yaw;
        if state.relative_yaw > 180.0 {
            state.relative_yaw -= 360.0;
        }
        if state.relative_yaw < -180.0 {
            state.relative_yaw += 360.0;
        }
        Ok(())
    }
}

impl GyroState {
    pub fn new() -> Self {
        GyroState {
            initial_yaw: f32::NAN,
            active: true,
            ..Default::default()
        }
    }

    pub fn read(&self) -> f32 {
        self.relative_yaw
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
