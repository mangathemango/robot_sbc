pub mod gyro;
pub mod maixcam;
pub mod stm32;

#[cfg(target_os = "linux")]
pub mod qr;

#[derive(Debug)]
pub enum DriverPort {
    Connected(Box<dyn serialport::SerialPort>),
    Disconnected(String),
}

impl DriverPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(e) => {
                return DriverPort::Disconnected(format!(
                    "Dotenv key {} fetch failed: {}",
                    dotenv_key, e
                ));
            }
        };
        let port = match serialport::new(&path, 115200).open() {
            Ok(port) => port,
            Err(e) => {
                return DriverPort::Disconnected(format!(
                    "Open driver port {} ({}) failed: {}",
                    dotenv_key, path, e
                ));
            }
        };
        DriverPort::Connected(port)
    }

    pub fn is_connected(&self) -> bool {
        match self {
            DriverPort::Connected(_) => true,
            _ => false,
        }
    }
}
