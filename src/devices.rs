pub mod gyro;
pub mod stm32;

#[derive(Debug)]
pub enum DriverPort {
    Active(Box<dyn serialport::SerialPort>),
    Inactive
}

impl DriverPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let gyro_path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(e) => {
                println!("GYRO_PORT not detected in .env: {}", e);
                return DriverPort::Inactive;
            }
        };
        let port = match serialport::new(&gyro_path, 115200).open() {
            Ok(port) => port,
            Err(e) => {
                println!("Failed to open GYRO_PORT ({}): {}", gyro_path, e);
                return  DriverPort::Inactive;
            }
        };
        DriverPort::Active(port)
    }
}