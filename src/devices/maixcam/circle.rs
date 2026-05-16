use glam::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct MaixcamCircle {
    pub position: Vec2,
    pub color: MaixcamCircleColor,
}

impl MaixcamCircle {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        
        let pos_x = u16::from_le_bytes([bytes[0], bytes[1]]) as f32 / 10000.0;
        let pos_y = u16::from_le_bytes([bytes[2], bytes[3]]) as f32 / 10000.0;
        let position = Vec2::new(pos_x, pos_y);
        let color = MaixcamCircleColor::from_id(bytes[4]);
        Self {
            position,
            color
        }
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MaixcamCircleColor {
    #[default]
    Unknown,
    Red,
    Green,
    Blue,
}

impl MaixcamCircleColor {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => MaixcamCircleColor::Red,
            2 => MaixcamCircleColor::Green,
            3 => MaixcamCircleColor::Blue,
            _ => MaixcamCircleColor::Unknown,
        }
    }
}