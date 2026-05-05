mod devices;
mod robot;
use robot::Robot;
use std::{thread, time::Duration};
use crate::devices::gyro::{GyroDriver, GyroState};
use std::sync::Arc;
use once_cell::sync::Lazy;

static ROBOT: Lazy<Robot> = Lazy::new(|| {Robot::new()});

fn main() {
    let handle = thread::spawn(|| gyro_fetcher());

    thread::spawn(|| {
        loop {
            let gyro = ROBOT.gyro_state.load();
            if gyro.is_active() {
                println!("Yaw: {}", gyro.read());
            }
            
        }
        
    });

    handle.join().unwrap();
    
}   

fn gyro_fetcher() {
    let mut driver = GyroDriver::new();
    let mut state = GyroState::new();

    loop {
        match driver.update_state(&mut state) {
            Ok(_) => (),
            Err(_) => {
                driver.reconnect();
                thread::sleep(Duration::from_millis(200));
            }
        };
        ROBOT.gyro_state.store(Arc::new(state.clone()))
    }
}
