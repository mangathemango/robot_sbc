pub mod gyro;

#[derive(Debug)]
pub enum DriverPort {
    Active(Box<dyn serialport::SerialPort>),
    Inactive
}