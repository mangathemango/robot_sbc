use crate::devices::gyro::GyroState;
use crate::devices::stm32::Stm32State;
use arc_swap::ArcSwap;

pub struct Robot {
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
        }
    }
}