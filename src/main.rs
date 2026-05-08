mod control;
mod debug;
mod devices;
mod math;
mod robot;
use crate::control::odometry::spawn_odometry_thread;
use crate::control::spawn_main_controller_thread;
use crate::debug::spawn_debug_thread;
use crate::devices::gyro::spawn_gyro_thread;
use crate::devices::maixcam::spawn_maixcam_thread;
use crate::devices::qr::spawn_qr_thread;
use crate::devices::stm32::{Stm32Controller, spawn_stm32_thread};

use once_cell::sync::Lazy;
use robot::Robot;
use std::sync::mpsc;

static ROBOT: Lazy<Robot> = Lazy::new(|| Robot::new());

fn main() {
    // Create mpsc (Multi-Producer, Single-Consumer) channel for multiple different threads
    // to send commands to the stm32 in the stm32_thread
    let (sender, receiver) = mpsc::channel();

    // Sender is set globally. Other threads can clone to control the STM32
    ROBOT
        .stm32_controller
        .set(Stm32Controller::new(sender))
        .expect("Unable to set STM32_CONTROLLER: {}");

    // Receiver is passed on to stm32 thread
    spawn_stm32_thread(receiver);
    spawn_gyro_thread();
    spawn_maixcam_thread();
    spawn_qr_thread();
    spawn_odometry_thread();
    spawn_main_controller_thread();

    // The Debug thread has to be the last thread spawned
    spawn_debug_thread();
}
