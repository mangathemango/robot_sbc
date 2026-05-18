#[cfg(target_os = "linux")]
mod linux;

#[cfg(not(target_os = "linux"))]
mod stub;

use std::{thread, time::Duration};

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
pub use stub::*;

use crate::ROBOT;

pub fn spawn_qr_thread() {
    std::thread::spawn(move || {
        let mut driver = QrDriver::new();
        let mut last_update = std::time::Instant::now();
        ROBOT.lock_qr_state().driver_is_connected = driver.is_connected();
        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_update);
            if dt < Duration::from_millis(1000) {
                thread::sleep(Duration::from_millis(100));
                continue;
            } 
            ROBOT.lock_qr_state().dt = dt;
            last_update = now;

            match driver.try_read() {
                Ok(Some(code)) => {
                    ROBOT.lock_qr_state().update(code);
                }
                Ok(None) => {}
                Err(msg) => {
                    driver.device = DriverHIDDevice::Disconnected(msg.clone());
                    ROBOT.lock_qr_state().error_msg = msg.clone();
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            ROBOT.lock_qr_state().driver_is_connected = driver.is_connected();
        }
    });
}
