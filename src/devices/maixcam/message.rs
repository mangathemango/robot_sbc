use crate::devices::{
    maixcam::circle::MaixcamCircle, 
    utils::SerialMessage
};

#[derive(Debug, Clone)]
pub enum MaixcamMessage {
    CircleData(Vec<MaixcamCircle>),
}

impl MaixcamMessage {
    pub fn from_id(id: u8) -> Option<MaixcamMessage> {
        match id {
            0x01 => Some(MaixcamMessage::CircleData(Vec::new())),
            _ => None,
        }
    }
}

impl SerialMessage for MaixcamMessage {
    const START_BYTE: u8 = 0x69;
    fn from_frame(frame: &[u8]) -> Result<Self, String> {
        if frame.len() < 4 {
            return Err("Frame too short".into());
        }

        let id = frame[1];
        let len = frame[2] as usize;

        let data = &frame[3..3 + len];

        if let Some(command) = MaixcamMessage::from_id(id) {
            match command {
                MaixcamMessage::CircleData(..) => {
                    let circles = data
                        .chunks_exact(5)
                        .map(|bytes| MaixcamCircle::from_bytes(bytes))
                        .collect();
                    Ok(MaixcamMessage::CircleData(circles))
                }
            }
        } else {
            Err("Unrecognised Maixcam command id".into())
        }
    }

    fn valid_id(id: u8) -> bool {
        Self::from_id(id).is_some()
    }
}
