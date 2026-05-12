use crate::control::ControllerState;
use crate::control::states::odometry::OdometryState;
use crate::devices::gyro::GyroState;
use crate::devices::maixcam::MaixcamState;
use crate::devices::qr::QrState;
use crate::devices::stm32::{Stm32Controller, Stm32State};
use arc_swap::ArcSwap;
use std::sync::OnceLock;

pub struct Robot {
    // Device states
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>,
    pub maixcam_state: ArcSwap<MaixcamState>,
    pub qr_state: ArcSwap<QrState>,

    // Control states
    pub odometry_state: ArcSwap<OdometryState>,
    pub controller_state: ArcSwap<ControllerState>,

    pub stm32_controller: OnceLock<Stm32Controller>,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
            maixcam_state: ArcSwap::from_pointee(MaixcamState::new()),
            qr_state: ArcSwap::from_pointee(QrState::new()),
            odometry_state: ArcSwap::from_pointee(OdometryState::default()),
            controller_state: ArcSwap::from_pointee(ControllerState::default()),

            stm32_controller: OnceLock::new(),
        }
    }

    pub fn get_stm32_controller(&self) -> Stm32Controller {
        self.stm32_controller
            .get()
            .expect("STM32 controller not initialized")
            .clone()
    }
}
