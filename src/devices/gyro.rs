use std::default;

/// Driver struct to read + parse data sent from the gyro
#[derive(Debug)]
pub struct GyroDriver {
    port: Box<dyn serialport::SerialPort>,
}

/// A data sample read from the gyroscope
#[derive(Debug, Default)]
pub struct GyroSample {
    yaw: f32,
    gy: f32,
    gz: f32,
}

/// Current gyroscope state
#[derive(Debug, Default)]
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
}

impl GyroDriver {
    pub fn new() -> Result<Self, String> {
        let gyro_port = dotenv::var("GYRO_PORT")
            .map_err(|e| format!("GYRO_PORT not detected in .env: {}", e))?;
        let port = serialport::new(&gyro_port, 115200)
            .open()
            .map_err(|e| format!("Failed to open GYRO_PORT ({}): {}", gyro_port, e))?;

        Ok(GyroDriver { port })
    }

    fn parse_frame(frame: &[u8]) -> GyroSample {
        let yaw = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;
        let gy = i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 2000.0;
        let gz = i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 2000.0;

        GyroSample { yaw, gy, gz }
    }

    pub fn get_sample(&mut self) -> Result<GyroSample, String> {
        let mut buffer = [0; 1];
        let mut frame = Vec::new();
        loop {
            match self.port.read_exact(&mut buffer) {
                Ok(_) => {}
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    return Err(format!("Gyro poll error: {}", e));
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

    pub fn update_state(&mut self, state: &mut GyroState) -> Result<(), String> {
        let sample = self.get_sample()?;
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
            ..Default::default()
        }
    }

    pub fn read(&self) -> f32 {
        self.relative_yaw
    }
}
