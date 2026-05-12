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

    pub fn rotate_to(&mut self, target_position: ClawServoPosition) {
        self.set_angle(target_position.to_angle());
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ClawServoPosition {
    #[default]
    Open,
    SoftOpen,
    Close,
    Custom(u8),
}

impl ClawServoPosition {
    pub fn to_angle(&self) -> u8 {
        match self {
            ClawServoPosition::Open => 30,
            ClawServoPosition::SoftOpen => 120,
            ClawServoPosition::Close => 180,
            ClawServoPosition::Custom(angle) => *angle,
        }
    }
}
