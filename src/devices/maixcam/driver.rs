use crate::devices::{DriverSerialPort, maixcam::{MAIXCAM_DOTENV_KEY, MAIXCAM_START_BYTE, sample::MaixcamSample}};

#[derive(Debug)]
pub struct MaixcamDriver {
    port: DriverSerialPort,
}

impl MaixcamDriver {
    pub fn new() -> Self {
        MaixcamDriver {
            port: DriverSerialPort::from_dotenv_key(MAIXCAM_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverSerialPort::from_dotenv_key(MAIXCAM_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn try_read_frame(&mut self) -> Result<MaixcamSample, String> {
        match &mut self.port {
            DriverSerialPort::Disconnected(msg) => Err(format!("Maixcam driver not active: {}", msg)),
            DriverSerialPort::Connected(port) => {
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
                                self.port = DriverSerialPort::Disconnected(
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