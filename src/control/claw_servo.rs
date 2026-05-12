#[derive(Debug, Clone, Copy, Default)]
pub struct ClawServoState {
    pub current_angle: u8,
}

impl ClawServoState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_angle(&mut self, target_angle: u8) {
        self.current_angle = target_angle
    }

    pub fn rotate_to(&mut self, target_position: ClawPosition) {
        self.set_angle(target_position.to_angle());
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClawPosition {
    #[default]
    Open,
    SoftOpen,
    Close,
    Custom(u8),
}

impl ClawPosition {
    pub fn to_angle(&self) -> u8 {
        match self {
            ClawPosition::Open => 30,
            ClawPosition::SoftOpen => 120,
            ClawPosition::Close => 180,
            ClawPosition::Custom(angle) => *angle,
        }
    }
}
