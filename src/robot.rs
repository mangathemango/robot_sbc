use crate::control::{Controller, ControllerState};
use crate::control::states::odometry::OdometryState;
use crate::devices::gyro::state::GyroState;
use crate::devices::maixcam::state::MaixcamState;
use crate::devices::qr::QrState;
use crate::devices::stm32::controller::Stm32Controller;
use crate::devices::stm32::state::Stm32State;
use arc_swap::{ArcSwap, Guard};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

pub struct Robot {
    // Device states
    gyro_state: ArcSwap<GyroState>,
    stm32_state: ArcSwap<Stm32State>,
    maixcam_state: ArcSwap<MaixcamState>,
    qr_state: ArcSwap<QrState>,

    // Control states
    odometry_state: Arc<Mutex<OdometryState>>,
    controller_state: ArcSwap<ControllerState>,

    stm32_controller: OnceLock<Stm32Controller>,
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

    pub fn get_gyro_state(&self) -> Guard<Arc<GyroState>> {
        self.gyro_state.load()
    }

    pub fn set_gyro_state(&self, state: GyroState) {
        self.gyro_state.store(Arc::new(state));
    }

    pub fn get_stm32_state(&self) -> Guard<Arc<Stm32State>> {
        self.stm32_state.load()
    }

    pub fn set_stm32_state(&self, state: Stm32State) {
        self.stm32_state.store(Arc::new(state));
    }


    pub fn get_maixcam_state(&self) -> Guard<Arc<MaixcamState>> {
        self.maixcam_state.load()
    }

    pub fn set_maixcam_state(&self, state: MaixcamState) {
        self.maixcam_state.store(Arc::new(state));
    }

    pub fn get_qr_state(&self) -> Guard<Arc<QrState>> {
        self.qr_state.load()
    }

    pub fn set_qr_state(&self, state: QrState) {
        self.qr_state.store(Arc::new(state));
    }


    pub fn get_odometry_state(&self) -> OdometryState {
        self.odometry_state.lock().unwrap().clone()
    }

    pub fn lock_odometry_state(&self) -> MutexGuard<'_, OdometryState> {
        self.odometry_state.lock().unwrap()
    }

    pub fn get_controller_state(&self) -> Guard<Arc<ControllerState>> {
        self.controller_state.load()
    }

    pub fn set_controller_state(&self, state: ControllerState) {
        self.controller_state.store(Arc::new(state));
    }

    pub fn get_stm32_controller(&self) -> Stm32Controller {
        self.stm32_controller
            .get()
            .expect("STM32 controller not initialized")
            .clone()
    }

    pub fn set_stm32_controller(&self, controller: Stm32Controller) {
        self.stm32_controller.set(controller).unwrap()
    }
}
