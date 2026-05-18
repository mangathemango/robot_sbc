mod control;
mod dashboard;
mod devices;
mod math;
mod robot;
mod scheduler;

use crate::control::states::odometry::spawn_odometry_thread;
use crate::dashboard::spawn_dashboard_thread;
use crate::devices::gyro::spawn_gyro_thread;
use crate::devices::maixcam::spawn_maixcam_thread;
use crate::devices::qr::spawn_qr_thread;
use crate::devices::stm32::spawn_stm32_thread;
use crate::scheduler::spawn_scheduler_thread;

use once_cell::sync::Lazy;
use robot::Robot;

// The global ROBOT variable used to share data across different threads
static ROBOT: Lazy<Robot> = Lazy::new(|| Robot::new());

fn main() {
    // DEVICE THREADS
    // Thread to retrieve and send serial data to the STM32. Updates ROBOT.stm32_state and provides ROBOT.get_stm32_controller()
    spawn_stm32_thread();

    // Thread to retrieve raw data from the HWTCT101 gyroscope. Updates ROBOT.gyro_data
    spawn_gyro_thread();

    // Thread to retrieve detected circle data from the maixcam. Updates ROBOT.maixcam_state
    spawn_maixcam_thread();

    // Thread to continuously read data from the QR code reader. Updates ROBOT.qr_state
    spawn_qr_thread();

    // CONTROL THREADS
    // Thread to estimate current position + movement of the robot. Updates ROBOT.odometry_state
    spawn_odometry_thread();

    // Thread to queue high level actions and sequences. Updates ROBOT.scheduler_state
    // spawn_scheduler_thread();

    // Thread to render TUI for debugging
    spawn_dashboard_thread();
}
