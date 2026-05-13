mod control;
mod debug;
mod devices;
mod math;
mod robot;
use crate::control::states::odometry::spawn_odometry_thread;
use crate::control::spawn_main_controller_thread;
use crate::debug::spawn_debug_thread;
use crate::devices::gyro::spawn_gyro_thread;
use crate::devices::maixcam::spawn_maixcam_thread;
use crate::devices::qr::spawn_qr_thread;
use crate::devices::stm32::spawn_stm32_thread;

use crate::devices::stm32::controller::Stm32Controller;

use once_cell::sync::Lazy;
use robot::Robot;
use std::sync::mpsc;

// The global ROBOT variable used to share data across different threads
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

    // DEVICE THREADS
    // Thread to retrieve and send serial data to the STM32. Updates ROBOT.stm32_state
    spawn_stm32_thread(receiver);       

    // Thread to retrieve raw data from the HWTCT101 gyroscope. Updates ROBOT.gyro_data
    spawn_gyro_thread();                 

    // Thread to retrieve detected circle data from the maixcam. Updates ROBOT.maixcam_state
    spawn_maixcam_thread();

    // Thread to continuously read data from the QR code reader. 
    spawn_qr_thread();

    // CONTROL THREADS
    // Thread to estimate current position + movement of the robot. Updates ROBOT.odometry_state
    spawn_odometry_thread();

    // Thread to queue high level actions and sequences. Updates ROBOT.controller_state
    spawn_main_controller_thread();
    
    // The Debug thread has to be the last thread spawned
    spawn_debug_thread();
}
