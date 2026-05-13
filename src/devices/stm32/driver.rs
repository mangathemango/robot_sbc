use crate::devices::{DriverSerialPort, stm32::{STM32_DOTENV_KEY, STM32_START_BYTE, commands::{PiToStm32Command, Stm32ToPiCommand}}};

#[derive(Debug)]
pub struct Stm32Driver {
    port: DriverSerialPort,
}

impl Stm32Driver {
    pub fn new() -> Self {
        Stm32Driver {
            port: DriverSerialPort::from_dotenv_key(STM32_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverSerialPort::from_dotenv_key(STM32_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn send_command(&mut self, command: PiToStm32Command) -> Result<usize, String> {
        let port = match &mut self.port {
            DriverSerialPort::Disconnected(msg) => {
                return Err(format!("Send command to STM32 failed: {}", msg).into());
            }
            DriverSerialPort::Connected(port) => port,
        };
        port.write(&command.to_packet_bytes())
            .map_err(|e| format!("Send command to STM32 failed: {}", e))
    }

    pub fn try_read_frame(&mut self) -> Result<Option<Stm32ToPiCommand>, String> {
        let port = match &mut self.port {
            DriverSerialPort::Disconnected(msg) => {
                return Err(format!("Read from STM32 failed: {}", msg).into());
            }
            DriverSerialPort::Connected(port) => port,
        };

        let mut buffer = [0u8; 1];
        let mut frame = Vec::<u8>::new();
        let mut idx: usize = 0;
        let mut total_len: usize = usize::MAX;

        loop {
            match port.read(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        return Ok(None);
                    } else {
                        self.port =
                            DriverSerialPort::Disconnected(format!("Read from STM32 failed: {}", e));
                        return Err(format!("Read from STM32 failed: {}", e));
                    }
                }
            }

            let byte = buffer[0];

            match idx {
                0 => {
                    if byte != STM32_START_BYTE {
                        continue;
                    }
                }

                1 => {
                    if Stm32ToPiCommand::from_id(byte).is_none() {
                        idx = 0;
                        frame.clear();
                        continue;
                    }
                }

                2 => {
                    let len = byte as usize;
                    total_len = 3 + len + 1; // START + ID + LEN + DATA + CHK
                }

                _ => {}
            }

            frame.push(byte);
            idx += 1;

            if idx == total_len {
                let checksum = frame[total_len - 1];

                let calc = frame[..total_len - 1].iter().fold(0u8, |acc, x| acc ^ x);

                if checksum != calc {
                    // bad frame, reset and continue scanning
                    idx = 0;
                    frame.clear();
                    continue;
                }

                let parsed = Stm32ToPiCommand::from_frame(frame)?;

                return Ok(Some(parsed));
            }
        }
    }
}