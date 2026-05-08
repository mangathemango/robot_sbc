use std::iter::Once;
use std::sync::mpsc::{Receiver, Sender};

use crate::control::motion::MotionState;
use crate::devices::gyro::GyroState;
use crate::devices::maixcam::MaixcamState;
use crate::devices::stm32::{PiToStm32Command, Stm32Controller, Stm32State};
use crate::devices::qr::{self, QrState};
use std::sync::{OnceLock, mpsc};
use arc_swap::ArcSwap;

pub struct Robot {
    pub gyro_state: ArcSwap<GyroState>,
    pub stm32_state: ArcSwap<Stm32State>,
    pub maixcam_state: ArcSwap<MaixcamState>,
    pub qr_state: ArcSwap<QrState>,
    pub motion_state: ArcSwap<MotionState>,

    pub stm32_controller:  OnceLock<Stm32Controller>
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            gyro_state: ArcSwap::from_pointee(GyroState::new()),
            stm32_state: ArcSwap::from_pointee(Stm32State::new()),
            maixcam_state: ArcSwap::from_pointee(MaixcamState::new()),
            qr_state: ArcSwap::from_pointee(QrState::new()),
            motion_state: ArcSwap::from_pointee(MotionState::default()),

            stm32_controller: OnceLock::new()
        }
    }

    pub fn get_stm32_controller(&self) -> Stm32Controller {
        self.stm32_controller
            .get()
            .expect("STM32 controller not initialized")
            .clone()
    }
}