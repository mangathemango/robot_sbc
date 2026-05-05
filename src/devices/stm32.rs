use crate::devices::DriverPort;
use glam::Vec2;
use std::sync::mpsc::Sender;

const STM32_DOTENV_KEY: &str = "STM32_PATH";

/// A packet of byte data will be formatted like this:
///     [[START] [COMMAND_ID] [LEN] [...DATA...] [CHK]]
/// (CHK = Checksum for data integrity validation, XOR all the bytes from [START] to the end of [...DATA...])
const START_BYTE: u8 = 0x67;

#[derive(Debug)]
pub enum PiToStm32Command {
    /// angle: 0-180
    SetYawServoAngle {
        angle: u8,
    },
    SetClawServoAngle {
        angle: u8,
    },
    /// text: a string of ASCII characters
    SetDisplayText {
        text: String,
    },

    /// Motor drivers will (hopefully) handle these two
    CalibrateVerticalArm {},
    CalibrateHorizontalArm {},

    /// position: values mapped from 0 (bottom/backwards) - 10000 (top/forwards)
    SetVerticalArmPosition {
        position: u16,
    },
    SetHorizontalArmPosition {
        position: u16,
    },

    Beep {},
    /// velocities: values mapped from -10000 (-max) - 10000 (max)
    /// Note: Only set wheel velocity to target velocity, do not auto ramp velocity back to 0
    SetWheelTargetVelocities {
        velocities: [i16; 4],
    },
}

#[derive(Debug)]
pub enum Stm32ToPiCommand {
    /// Actual wheel velocities from motor encoders
    SendActualWheelVelocities { velocities: [i16; 4] },
    /// running: 0x00 for false, 0x01 for true. Will set to 1 when button on STM32 is clicked
    SetRunningFlag { running: u8 },
}

#[derive(Debug)]
pub struct Stm32Driver {
    port: DriverPort,
}

#[derive(Debug)]
pub struct Stm32Controller {
    tx: Sender<PiToStm32Command>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32State {
    running: bool,
    // Movements
    actual_wheel_velocities: [i16; 4],
    actual_velocity: Vec2,
    actual_omega: f32,

    estimated_position: Vec2, // Estimated position based on accumulated actual velocities
}

impl PiToStm32Command {
    fn id(&self) -> u8 {
        match self {
            PiToStm32Command::SetYawServoAngle { .. } => 0x01,
            PiToStm32Command::SetClawServoAngle { .. } => 0x02,
            PiToStm32Command::SetDisplayText { .. } => 0x03,
            PiToStm32Command::CalibrateVerticalArm { .. } => 0x04,
            PiToStm32Command::CalibrateHorizontalArm { .. } => 0x05,
            PiToStm32Command::SetVerticalArmPosition { .. } => 0x06,
            PiToStm32Command::SetHorizontalArmPosition { .. } => 0x07,
            PiToStm32Command::Beep { .. } => 0x08,
            PiToStm32Command::SetWheelTargetVelocities { .. } => 0x09,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.push(START_BYTE);

        let id = self.id();
        buf.push(id);

        // reserve space for length (we fill later)
        buf.push(0);

        match self {
            PiToStm32Command::SetYawServoAngle { angle } => {
                buf.push(*angle);
            }

            PiToStm32Command::SetDisplayText { text } => {
                let bytes = text.as_bytes();

                // ⚠ enforce max length
                let len = bytes.len().min(32);
                buf.extend(&bytes[..len]);
            }

            PiToStm32Command::SetWheelTargetVelocities { velocities } => {
                velocities.iter().for_each(|v| buf.extend(v.to_le_bytes()));
            }

            _ => {}
        }

        // fill in length (data only)
        let data_len = buf.len() - 3;
        buf[2] = data_len as u8;

        // checksum (XOR)
        let checksum = buf.iter().fold(0u8, |acc, x| acc ^ x);
        buf.push(checksum);

        buf
    }
}

impl Stm32ToPiCommand {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0x51 => Some(Stm32ToPiCommand::SendActualWheelVelocities { velocities: [0; 4] }),
            0x52 => Some(Stm32ToPiCommand::SetRunningFlag { running: 0x0 }),
            _ => None,
        }
    }

    pub fn from_frame(frame: Vec<u8>) -> Result<Self, String> {
        if frame.len() < 4 {
            return Err("Frame too short".into());
        }

        let id = frame[1];
        let len = frame[2] as usize;

        let data = &frame[3..3 + len];

        match id {
            0x51 => {
                if data.len() != 8 {
                    return Err("Invalid velocity data length".into());
                }

                let mut velocities = [0i16; 4];
                for i in 0..4 {
                    velocities[i] = i16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
                }

                Ok(Stm32ToPiCommand::SendActualWheelVelocities { velocities })
            }

            0x52 => {
                if data.len() != 1 {
                    return Err("Invalid running flag length".into());
                }

                Ok(Stm32ToPiCommand::SetRunningFlag { running: data[0] })
            }

            _ => Err(format!("Unknown command ID: {}", id)),
        }
    }
}

impl Stm32Driver {
    pub fn new() -> Self {
        Stm32Driver {
            port: DriverPort::from_dotenv_key(STM32_DOTENV_KEY),
        }
    }

    pub fn send_command(&mut self, command: PiToStm32Command) -> Result<usize, String> {
        let port = match &mut self.port {
            DriverPort::Inactive => {
                return Err("Send command to STM32 failed: STM32 driver not active".into());
            }
            DriverPort::Active(port) => port,
        };
        port.write(&command.to_bytes())
            .map_err(|e| format!("Send command to STM32 failed: {}", e))
    }

    pub fn try_read_frame(&mut self) -> Result<Option<Stm32ToPiCommand>, String> {
        let port = match &mut self.port {
            DriverPort::Inactive => {
                return Err("Read from STM32 failed: STM32 driver not active".into());
            }
            DriverPort::Active(port) => port,
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
                        return Err(format!("Read from STM32 failed: {}", e));
                    }
                }
            }

            let byte = buffer[0];

            match idx {
                0 => {
                    if byte != START_BYTE {
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

impl Stm32State {
    pub fn new() -> Stm32State {
        Stm32State::default()
    }

    pub fn update(&mut self, command: Stm32ToPiCommand) {
        match command {
            Stm32ToPiCommand::SendActualWheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
                let v = velocities;
                self.actual_velocity.x =    ( v[0] + v[1] + v[2] + v[3]) as f32 / 4.0;
                self.actual_velocity.y =    (-v[0] + v[1] + v[2] - v[3]) as f32 / 4.0;
                self.actual_omega =         (-v[0] + v[1] - v[2] + v[3]) as f32 / 4.0;
            },
            Stm32ToPiCommand::SetRunningFlag { running } => {
                self.running = running != 0;
            }
        };
    }
}

impl Stm32Controller {
    pub fn new(tx: Sender<PiToStm32Command>) -> Self {
        Self { tx }
    }
    pub fn send(&self, cmd: PiToStm32Command) {
        let _ = self.tx.send(cmd);
    }
    pub fn beep(&self) {
        self.send(PiToStm32Command::Beep {});
    }
    pub fn set_yaw_servo(&self, angle: u8) {
        self.send(PiToStm32Command::SetYawServoAngle { angle });
    }

    pub fn set_claw_servo(&self, angle: u8) {
        self.send(PiToStm32Command::SetClawServoAngle { angle });
    }

    pub fn set_wheel_velocities(&self, v: [i16; 4]) {
        self.send(PiToStm32Command::SetWheelTargetVelocities {
            velocities: v,
        });
    }
    pub fn set_velocity(&self, v: Vec2, omega: f32) {
        let [v1, v2, v3, v4] = self.body_to_wheels(v, omega);

        self.set_wheel_velocities([v1, v2, v3, v4]);
    }

    fn body_to_wheels(&self, v: Vec2, omega: f32) -> [i16; 4] {
        let vx = v.x;
        let vy = v.y;

        let v1 = vx - vy -  omega;
        let v2 = vx + vy +  omega;
        let v3 = vx + vy -  omega;
        let v4 = vx - vy +  omega;

        [
            v1 as i16,
            v2 as i16,
            v3 as i16,
            v4 as i16,
        ]
    }
}