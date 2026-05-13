use crate::devices::stm32::STM32_START_BYTE;

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
