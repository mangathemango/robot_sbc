mod devices;
mod robot;
use crate::devices::gyro::{GyroDriver, GyroState};
use crate::devices::stm32::{PiToStm32Command, Stm32Controller, Stm32Driver, Stm32State};
use once_cell::sync::Lazy;
use robot::Robot;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::{thread, time::Duration};

static ROBOT: Lazy<Robot> = Lazy::new(|| Robot::new());

fn main() {
    let (tx, rx) = mpsc::channel();
    let stm32= Stm32Controller::new(tx);

    // spawn_gyro_thread();
    spawn_stm32_thread(rx);
    loop {
        let gyro = ROBOT.gyro_state.load();
        stm32.set_claw_servo(10);
        thread::sleep(Duration::from_millis(1000));
        // if gyro.is_active() {
        //     println!("Yaw: {}", gyro.read());
        // }
    }


}

fn spawn_gyro_thread() {
    std::thread::spawn(move || {
        let mut driver = GyroDriver::new();
        let mut state = GyroState::new();

        loop {
            match driver.get_sample() {
                Ok(sample) => {
                    state.update(sample);
                    state.set_activity(true);
                }
                Err(_) => {
                    state.set_activity(false);
                    driver.reconnect();
                    thread::sleep(Duration::from_millis(200));
                }
            };
            ROBOT.gyro_state.store(Arc::new(state.clone()))
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
                    state.update( command);
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("STM32 error: {}", e);
                    // maybe reconnect
                }
            }
            ROBOT.stm32_state.store(Arc::new(state.clone()))
        }
    });
}
