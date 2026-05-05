#[derive(Debug, Default)]
pub struct Gyro {
    port: Option<Box<dyn serialport::SerialPort>>,
    buffer: [u8; 1],
    frame: Vec<u8>,
    /// The first recorded yaw for relative yaw calculation
    initial_yaw: f32,

    /// Current yaw
    current_yaw: f32,

    /// Relative yaw (with respect to initial_yaw)
    relative_yaw: f32,

    /// y angular acceleration
    gy: f32,

    /// z angular acceleration
    gz: f32,
}

impl Gyro {
    pub fn new() -> Result<Self, String> {
        let gyro_port = dotenv::var("GYRO_PORT")
            .map_err(|e| format!("GYRO_PORT not detected in .env: {}", e))?;
        let port = serialport::new(&gyro_port, 115200)
            .open()
            .map_err(|e| format!("Failed to open GYRO_PORT ({}): {}", gyro_port, e))?;

        Ok(Gyro {
            port: Some(port),
            initial_yaw: f32::MAX,
            ..Default::default()
        })
    }

    pub fn update(&mut self) -> Result<(), String> {
        let port = self.port.as_mut().ok_or("Gyro port not initialized")?;

        loop {
            match port.read_exact(&mut self.buffer) {
                Ok(n) => {
 
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    return Err(format!("Gyro poll error: {}", e));
                }
            }

            let byte = self.buffer[0];

            // Always push
            self.frame.push(byte);

            // Keep frame from growing infinitely
            if self.frame.len() > 22 {
                self.frame.remove(0);
            }

            // Try to detect valid frame
            if self.frame.len() >= 2 {
                // Check header alignment
                if self.frame[0] != 0x55 {
                    self.frame.remove(0);
                    continue;
                }

                if self.frame[1] != 0x53 {
                    self.frame.remove(0);
                    continue;
                }
            }

            if self.frame.len() == 22 {
                let (yaw, gy, gz) = Self::parse_frame(&self.frame);

                self.current_yaw = yaw;
                self.gy = gy;
                self.gz = gz;

                if self.initial_yaw == f32::MAX {
                    self.initial_yaw = yaw;
                }

                self.relative_yaw = yaw - self.initial_yaw;

                // Normalize to [-180, 180]
                if self.relative_yaw > 180.0 {
                    self.relative_yaw -= 360.0;
                }
                if self.relative_yaw < -180.0 {
                    self.relative_yaw += 360.0;
                }

                // Reset for next frame
                self.frame.clear();
                return Ok(());
            }
        }
    }

    pub fn update_and_read(&mut self) -> Result<(f32, f32, f32), String> {
        self.update()?;
        Ok((self.relative_yaw, self.gy, self.gz))
    }

    pub fn read(&self) -> (f32, f32, f32) {
        (self.relative_yaw, self.gy, self.gz)
    }

    fn parse_frame(frame: &[u8]) -> (f32, f32, f32) {
        let yaw = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;
        let gy = i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 2000.0;
        let gz = i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 2000.0;

        (yaw, gy, gz)
    }
}
