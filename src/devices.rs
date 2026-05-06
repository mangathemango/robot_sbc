pub mod gyro;
pub mod stm32;
pub mod maixcam;

#[derive(Debug)]
pub enum DriverPort {
    Active(Box<dyn serialport::SerialPort>),
    Inactive
}

impl DriverPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let gyro_path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(_) => {
                return DriverPort::Inactive;
            }
        };
        let port = match serialport::new(&gyro_path, 115200).open() {
            Ok(port) => port,
            Err(_) => {
                return  DriverPort::Inactive;
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