use crate::control::motion::MotionState;
use crate::devices::gyro::GyroState;
use crate::devices::maixcam::MaixcamState;
use crate::devices::stm32::Stm32State;
use crate::devices::qr::{self, QrState};
use arc_swap::ArcSwap;

pub struct Robot {
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>,
    pub maixcam_state: ArcSwap<MaixcamState>,
    pub qr_state: ArcSwap<QrState>,
    pub motion_state: ArcSwap<MotionState>
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
            maixcam_state: ArcSwap::from_pointee(MaixcamState::new()),
            qr_state: ArcSwap::from_pointee(QrState::new()),
            motion_state: ArcSwap::from_pointee(MotionState::default())
        
        }
    }
}