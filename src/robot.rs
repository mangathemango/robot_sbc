use crate::devices::gyro::GyroState;
use arc_swap::ArcSwap;

pub struct Robot {
    pub gyro_state: ArcSwap<GyroState>,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new())
        }
    }
}