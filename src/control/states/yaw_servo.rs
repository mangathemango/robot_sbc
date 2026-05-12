#[derive(Debug, Clone, Copy, Default)]
pub struct YawServoState {
    pub current_angle: u8,
}

impl YawServoState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_angle(&mut self, target_angle: u8) {
        self.current_angle = target_angle;
    }

    pub fn rotate_to(&mut self, target_position: ArmPosition) {
        self.set_angle(target_position.to_angle());
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArmPosition {
    #[default]
    Middle,
    Left,
    Right,
    StorageLeft,
    StorageMiddle,
    StorageRight,
    Custom(u8),
}

impl ArmPosition {
    pub fn to_angle(&self) -> u8 {
        match self {
            ArmPosition::Middle => 60,
            ArmPosition::Left => 120,
            ArmPosition::Right => 0,
            ArmPosition::StorageLeft => 50,
            ArmPosition::StorageMiddle => 60,
            ArmPosition::StorageRight => 70,
            ArmPosition::Custom(angle) => *angle,
        }
    }
}
