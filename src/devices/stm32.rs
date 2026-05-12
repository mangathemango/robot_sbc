use crate::ROBOT;
use crate::control::claw_servo::ClawServoState;
use crate::control::yaw_servo::YawServoState;
use crate::devices::DriverPort;
use crate::math::MecanumVelocities;
use crate::math::Twist;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::time::Instant;
use std::vec;

const STM32_DOTENV_KEY: &str = "STM32_PATH";

/// A packet of byte data will be formatted like this:
///     [[START] [COMMAND_ID] [LEN] [...DATA...] [CHK]]
/// (CHK = Checksum for data integrity validation, XOR all the bytes from [START] to the end of [...DATA...])
const STM32_START_BYTE: u8 = 0x67;

pub fn spawn_stm32_thread(rx: Receiver<PiToStm32Command>) {
    std::thread::spawn(move || {
        let mut driver = Stm32Driver::new();
        let mut state = Stm32State::new();
        let mut last_update = std::time::Instant::now();
        let mut target_mecanum_velocities = MecanumVelocities::default();
        loop {
            let now = std::time::Instant::now();
            state.dt = now.duration_since(last_update);
            last_update = now;

            // 🟣 1. Handle outgoing commands
            match rx.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        PiToStm32Command::SetWheelTargetVelocities { velocities } => {
                            target_mecanum_velocities = MecanumVelocities::from_array(velocities.map(|v| v as f32));
                        }
                        PiToStm32Command::SetClawServoAngle { angle } => {
                            state.claw_servo_state.set_angle(angle);
                        }
                        PiToStm32Command::SetYawServoAngle { angle } => {
                            state.yaw_servo_state.set_angle(angle);
                        }
                        _ => (),
                    }
                    let _ = driver.send_command(cmd);
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }

            // 🔵 2. Handle incoming data
            match driver.try_read_frame() {
                Ok(Some(command)) => {
                    state.update(command);
                }
                Ok(None) => {}
                Err(_) => {
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }

            let current_mecanum_velocities = MecanumVelocities::from_array(
                state.actual_wheel_velocities.map(|v| v as f32),
            );
            let simulated_mecanum_velocities = current_mecanum_velocities
                .simulate_mecanum_response(target_mecanum_velocities, state.dt);
            state.actual_wheel_velocities =
                simulated_mecanum_velocities.to_array().map(|v| v as i16);
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}

#[derive(Debug)]
pub struct Stm32Driver {
    port: DriverPort,
}

impl Stm32Driver {
    pub fn new() -> Self {
        Stm32Driver {
            port: DriverPort::from_dotenv_key(STM32_DOTENV_KEY),
        }
    }

    pub fn reconnect(&mut self) {
        self.port = DriverPort::from_dotenv_key(STM32_DOTENV_KEY);
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_connected()
    }

    pub fn send_command(&mut self, command: PiToStm32Command) -> Result<usize, String> {
        let port = match &mut self.port {
            DriverPort::Disconnected(msg) => {
                return Err(format!("Send command to STM32 failed: {}", msg).into());
            }
            DriverPort::Connected(port) => port,
        };
        port.write(&command.to_packet_bytes())
            .map_err(|e| format!("Send command to STM32 failed: {}", e))
    }

    pub fn try_read_frame(&mut self) -> Result<Option<Stm32ToPiCommand>, String> {
        let port = match &mut self.port {
            DriverPort::Disconnected(msg) => {
                return Err(format!("Read from STM32 failed: {}", msg).into());
            }
            DriverPort::Connected(port) => port,
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
                            DriverPort::Disconnected(format!("Read from STM32 failed: {}", e));
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

/// A controller struct used to send Pi to STM32 commands from other threads
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Stm32Controller {
    tx: Sender<PiToStm32Command>,
}

#[allow(dead_code)]
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
        self.send(PiToStm32Command::SetWheelTargetVelocities { velocities: v });
    }

    pub fn set_twist(&self, t: Twist) {
        self.set_wheel_velocities(
            MecanumVelocities::from_twist(t)
                .normalize()
                .to_array()
                .map(|v| (v * 10000.0) as i16),
        );
    }
}

/// A struct representing the current states polled from the Stm32
#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32State {
    pub driver_is_connected: bool,
    pub start_flag: bool,

    pub yaw_servo_state: YawServoState,
    pub claw_servo_state: ClawServoState,
    // Movement
    pub actual_wheel_velocities: [i16; 4],
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl Stm32State {
    pub fn new() -> Stm32State {
        Stm32State::default()
    }

    pub fn update(&mut self, command: Stm32ToPiCommand) {
        match command {
            Stm32ToPiCommand::SendActualWheelVelocities { velocities } => {
                self.actual_wheel_velocities = velocities;
            }
            Stm32ToPiCommand::SetRunningFlag { running } => {
                self.start_flag = running != 0;
            }
        };
    }

    pub fn publish(&self) {
        ROBOT.stm32_state.store(Arc::new(self.clone()));
    }
}

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
    CalibrateVerticalArm,
    CalibrateHorizontalArm,

    /// position: values mapped from 0 (bottom/backwards) - 10000 (top/forwards)
    SetVerticalArmPosition {
        position: u16,
    },
    SetHorizontalArmPosition {
        position: u16,
    },

    Beep,
    /// velocities: values mapped from -10000 (-max) - 10000 (max)
    /// Note: Only set wheel velocity to target velocity, do not auto ramp velocity back to 0
    SetWheelTargetVelocities {
        velocities: [i16; 4],
    },
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

    pub fn to_packet_bytes(&self) -> Vec<u8> {
        let data = self.to_data_bytes();
        let len = data.len() as u8;
        let mut packet = vec![
            STM32_START_BYTE, // START
            self.id(),        // ID
            len,              // LEN
        ];

        packet.extend(&data); // DATA

        // checksum (XOR)
        let checksum = packet.iter().fold(0u8, |acc, x| acc ^ x);
        packet.push(checksum);

        packet
    }

    pub fn to_data_bytes(&self) -> Vec<u8> {
        match self {
            PiToStm32Command::SetYawServoAngle { angle } => {
                vec![*angle]
            }

            PiToStm32Command::SetDisplayText { text } => text.as_bytes().to_vec(),

            PiToStm32Command::SetWheelTargetVelocities { velocities } => velocities
                .iter()
                .map(|v| v.to_le_bytes())
                .flatten()
                .collect(),

            _ => Vec::new(),
        }
    }

    pub fn to_bytes_string(&self) -> String {
        let bytes = self.to_packet_bytes();

        let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("0x{:02X}", b)).collect();

        format!("[{}]", hex_bytes.join(", "))
    }
}

#[derive(Debug)]
pub enum Stm32ToPiCommand {
    /// Actual wheel velocities from motor encoders
    SendActualWheelVelocities { velocities: [i16; 4] },
    /// running: 0x00 for false, 0x01 for true. Will set to 1 when button on STM32 is clicked
    SetRunningFlag { running: u8 },
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
