mod devices;
mod robot;
use robot::Robot;
use std::thread;
use crate::devices::gyro::{GyroDriver, GyroState};
use std::sync::Arc;
use once_cell::sync::Lazy;

static ROBOT: Lazy<Robot> = Lazy::new(|| {Robot::new()});

fn main() {
    let handle = thread::spawn(|| {
        let mut driver = GyroDriver::new();
        let mut state = GyroState::new();

        loop {
            match driver.update_state(&mut state) {
                Ok(_) =>    ROBOT.gyro_state.store(Arc::new(state.clone())),
                Err(e) => println!("Failed to update gyro state: {}", e)
            }
        }
    });

    thread::spawn(|| {
        loop {
            let guard = ROBOT.gyro_state.load();
            println!("Yaw: {}", guard.read());
        }
        
    });

    handle.join().unwrap();
    
}   
