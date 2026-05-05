use crate::devices::gyro::GyroState;
use crate::devices::maixcam::MaixcamState;
use crate::devices::stm32::Stm32State;
use arc_swap::ArcSwap;

pub struct Robot {
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>,
    pub maixcam_state: ArcSwap<MaixcamState>
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
            maixcam_state: ArcSwap::from_pointee(MaixcamState::new())
        }
    }
}