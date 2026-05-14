/// An enum containing a Serial port if connected, or an error message when disconnected
#[derive(Debug)]
pub enum DriverSerialPort {
    Connected(Box<dyn serialport::SerialPort>),
    Disconnected(String),
}

impl DriverSerialPort {
    pub fn from_dotenv_key(dotenv_key: &str) -> Self {
        let path = match dotenv::var(dotenv_key) {
            Ok(path) => path,
            Err(e) => {
                return DriverSerialPort::Disconnected(format!(
                    "Dotenv key {} fetch failed: {}",
                    dotenv_key, e
                ));
            }
        };
        let port = match serialport::new(&path, 115200).open() {
            Ok(port) => port,
            Err(e) => {
                return DriverSerialPort::Disconnected(format!(
                    "Open driver port {} ({}) failed: {}",
                    dotenv_key, path, e
                ));
            }
        };
        DriverSerialPort::Connected(port)
    }

    pub fn is_connected(&self) -> bool {
        match self {
            DriverSerialPort::Connected(_) => true,
            _ => false,
        }
    }
}
