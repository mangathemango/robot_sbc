#[derive(Debug, Clone, Copy, Default)]
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