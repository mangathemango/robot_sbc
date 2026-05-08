#[cfg(target_os = "linux")]
mod linux;

#[cfg(not(target_os = "linux"))]
mod stub;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
pub use stub::*;

use crate::ROBOT;
use std::sync::Arc;

pub fn spawn_qr_thread() {
    std::thread::spawn(move || {
        let mut driver = QrDriver::new();
        let mut state = QrState::new();
        let mut last_update = std::time::Instant::now();
        state.driver_is_connected = driver.is_connected();
        state.publish();
        loop {
            let now = std::time::Instant::now();
            state.dt = now.duration_since(last_update);
            last_update = now;

            match driver.try_read() {
                Ok(Some(code)) => {
                    state.code = code.clone();
                }
                Ok(None) => {}
                Err(msg) => {
                    driver.device = DriverHIDDevice::Disconnected(msg.clone());
                    state.error_msg = msg.clone();
                    driver.reconnect();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            state.driver_is_connected = driver.is_connected();
            state.publish();
        }
    });
}
