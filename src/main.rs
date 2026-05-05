mod devices;

use crate::devices::gyro::*;
fn main() -> Result<(), String>{
    let mut gyro_driver = GyroDriver::new()?;
    let mut gyro_state = GyroState::new();
    loop {
        gyro_driver.update_state(&mut gyro_state)?;
        println!("Yaw: {}", gyro_state.read());
    }
}   
