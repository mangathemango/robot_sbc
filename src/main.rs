mod debug;
mod devices;
mod robot;
mod math;
mod control;
use crate::control::spawn_control_thread;
use crate::devices::gyro::spawn_gyro_thread;
use crate::devices::maixcam::spawn_maixcam_thread;
use crate::devices::qr::spawn_qr_thread;
use crate::devices::stm32::{Stm32Controller, spawn_stm32_thread};
use crate::debug::spawn_debug_thread;

use std::sync::mpsc;
use once_cell::sync::Lazy;
use robot::Robot;

static ROBOT: Lazy<Robot> = Lazy::new(|| Robot::new());

fn main() {
    
    let (tx, rx) = mpsc::channel();
    let stm32 = Stm32Controller::new(tx);

    spawn_stm32_thread(rx);
    spawn_gyro_thread();
    spawn_maixcam_thread();
    spawn_qr_thread();
    spawn_debug_thread();    
    spawn_control_thread();
}

