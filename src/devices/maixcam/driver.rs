use crate::devices::{
    maixcam::{MAIXCAM_DOTENV_KEY, message::MaixcamMessage},
    utils::{DriverSerialPort, SerialDecoder},
};

#[derive(Debug)]
pub struct MaixcamDriver {
    port: DriverSerialPort,
    decoder: SerialDecoder<MaixcamMessage>
}

impl MaixcamDriver {
    pub fn new() -> Self {
        MaixcamDriver {
            port: DriverSerialPort::from_dotenv_key(MAIXCAM_DOTENV_KEY),
            decoder: SerialDecoder::new()
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverSerialPort::from_dotenv_key(MAIXCAM_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn try_read_frame(&mut self) -> Result<Vec<MaixcamMessage>, String> {
        let port = match &mut self.port {
            DriverSerialPort::Disconnected(msg) => {
                return Err(format!("Maixcam driver not active: {}", msg))
            }
            DriverSerialPort::Connected(port) => {
                port
            }
        };

        let mut buffer = [0u8; 256];

        let mut result = Vec::new();

        match port.read(&mut buffer) {
            Ok(n) => {
                for byte in &buffer[..n] {
                    if let Some(msg) = self.decoder.push_byte(*byte) {
                        result.push(msg);
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    return Ok(Vec::new());
                } else {
                    self.port =
                        DriverSerialPort::Disconnected(format!("Read from Maixcam failed: {}", e));
                    return Err(format!("Read from Maixcam failed: {}", e));
                }
            }
        }
        Ok(result)
    }
}
