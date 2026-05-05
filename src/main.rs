mod devices;

use crate::devices::gyro::Gyro;
fn main() {
    let mut gyro = Gyro::new().expect("Penis");
    loop {
        let (yaw, gy, gz) = gyro
            .update_and_read()
            .expect("poll error");
        println!("yaw: {:.2}\tgy: {:.2}\tgz: {:.2}", yaw, gy, gz);
    }
}