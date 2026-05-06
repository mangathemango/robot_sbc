pub mod gyro;
pub mod stm32;
pub mod maixcam;

#[derive(Debug)]
pub enum DriverPort {
    Active(Box<dyn serialport::SerialPort>),
    Inactive(String)
}

impl DriverPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(e) => {
                return DriverPort::Inactive(format!("Dotenv key {} fetch failed: {}", dotenv_key, e));
            }
        };
        let port = match serialport::new(&path, 115200).open() {
            Ok(port) => port,
            Err(e) => {
                return  DriverPort::Inactive(format!("Open driver port {} ({}) failed: {}", dotenv_key, path, e));
            }
        };
        DriverPort::Active(port)
    }

    pub fn is_active(&self) -> bool {
        match self {
            DriverPort::Active(_) => true,
            _ => false
        }
    }
}