use crate::control::ControllerState;
use crate::control::states::odometry::OdometryState;
use crate::devices::gyro::state::GyroState;
use crate::devices::maixcam::state::MaixcamState;
use crate::devices::qr::QrState;
use crate::devices::stm32::controller::Stm32Controller;
use crate::devices::stm32::state::Stm32State;
use arc_swap::ArcSwap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

pub struct Robot {
    // Device states
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>,
    pub maixcam_state: ArcSwap<MaixcamState>,
    pub qr_state: ArcSwap<QrState>,

    // Control states
    pub odometry_state: Arc<Mutex<OdometryState>>,
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
            odometry_state: Arc::new(Mutex::new(OdometryState::default())),
            controller_state: ArcSwap::from_pointee(ControllerState::default()),

            stm32_controller: OnceLock::new(),
        }
    }

    pub fn get_odometry_state(&self) -> OdometryState {
        self.odometry_state.lock().unwrap().clone()
    }

    pub fn lock_odometry_state(&self) -> MutexGuard<'_, OdometryState> {
        self.odometry_state.lock().unwrap()
    }

    pub fn get_stm32_controller(&self) -> Stm32Controller {
        self.stm32_controller
            .get()
            .expect("STM32 controller not initialized")
            .clone()
    }
}
