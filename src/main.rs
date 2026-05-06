mod debug;
mod devices;
mod robot;
use crate::devices::gyro::{GyroDriver, GyroState};
use crate::devices::maixcam::{MaixcamDriver, MaixcamState};
use crate::devices::stm32::{PiToStm32Command, Stm32Controller, Stm32Driver, Stm32State};
use once_cell::sync::Lazy;
use robot::Robot;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};

static ROBOT: Lazy<Robot> = Lazy::new(|| Robot::new());

fn main() {
    let (tx, rx) = mpsc::channel();
    let stm32 = Stm32Controller::new(tx);

    spawn_stm32_thread(rx);
    spawn_gyro_thread();
    spawn_maixcam_thread();
    debug::display::start();
    loop {}
}

fn spawn_gyro_thread() {
    std::thread::spawn(move || {
        let mut driver = GyroDriver::new();
        let mut state = GyroState::new();

        loop {
            match driver.try_read_frame() {
                Ok(sample) => {
                    state.error_msg = None;
                    state.update(sample);
                }
                Err(msg) => {
                    state.error_msg = Some(msg);
                    driver.reconnect();
                }
            };
            state.driver_is_active = driver.is_active();
            ROBOT.gyro_state.store(Arc::new(state.clone()));
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
}

pub fn spawn_stm32_thread(rx: Receiver<PiToStm32Command>) {
    std::thread::spawn(move || {
        let mut driver = Stm32Driver::new();
        let mut state = Stm32State::new();

        loop {
            // 🟣 1. Handle outgoing commands
            match rx.try_recv() {
                Ok(cmd) => {
                    let _ = driver.send_command(cmd);
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }

            // 🔵 2. Handle incoming data
            match driver.try_read_frame() {
                Ok(Some(command)) => {
                    state.update(command);
                }
                Ok(None) => {}
                Err(_) => {
                    driver.reconnect();
                }
            }
            state.driver_is_active = driver.is_active();
            ROBOT.stm32_state.store(Arc::new(state.clone()));
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
}

pub fn spawn_maixcam_thread() {
    std::thread::spawn(move || {
        let mut driver = MaixcamDriver::new();
        let mut state = MaixcamState::new();

        loop {
            match driver.try_read_frame() {
                Ok(sample) => {
                    state.update(sample);
                }
                Err(_) => {
                    driver.reconnect();
                }
            }
            state.driver_is_active = driver.is_active();
            ROBOT.maixcam_state.store(Arc::new(state.clone()));
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
}
