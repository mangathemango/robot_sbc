use std::{thread, time::Duration};

#[derive(Debug)]
pub enum DriverHIDDevice {
    Disconnected(String),
}

impl DriverHIDDevice {
    pub fn is_connected(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct QrDriver {
    pub device: DriverHIDDevice,
}

impl QrDriver {
    pub fn new() -> Self {
        QrDriver {
            device: DriverHIDDevice::Disconnected(
                "QR not supported on this OS".to_string(),
            ),
        }
    }

    pub fn reconnect(&mut self) {
        // still disconnected, life is pain
    }

    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn try_read(&mut self) -> Result<Option<String>, String> {
        thread::sleep(Duration::from_millis(100));
        Ok(None)
    }
}

#[derive(Debug, Clone, Default)]
pub struct QrState {
    pub driver_is_connected: bool,
    pub code: String,
    pub error_msg: String,
}

impl QrState {
    pub fn new() -> Self {
        Self {
            error_msg: "The qr code module depends on evdev to read data, which is a crate only built for Linux OS".into(),
            ..QrState::default()
        }
    }
}