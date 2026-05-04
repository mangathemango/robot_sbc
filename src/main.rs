use serialport::SerialPort;
use std::io::Read;
use std::time::Duration;
use std::thread::sleep;
fn main() {
    
    let mut port = serialport::new("COM8", 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    // println!("Calibrating...");
    // let cmd = [0xFF, 0xAA, 0x69, 0x88, 0xB5];
    // port.write_all(&cmd).unwrap();
    // let cmd = [0xFF, 0xAA, 0x01, 0x01, 0x00];
    // port.write_all(&cmd).unwrap();
    // sleep(Duration::from_secs(4));
    // let cmd = [0xFF, 0xAA, 0x01, 0x00, 0x00];
    // port.write_all(&cmd).unwrap();

    // let cmd = [0xFF, 0xAA, 0x00, 0x00, 0x00];
    // port.write_all(&cmd).unwrap();

    let mut buffer = [0u8; 1];
    let mut frame: Vec<u8> = Vec::new();

    let mut start_yaw = None;
    let mut yaw:f32 = 0.0;
    let mut gy :f32 = 0.0;
    let mut gz :f32 = 0.0;
    loop {
        
        if let Ok(_) = port.read_exact(&mut buffer) {
            let byte = buffer[0];

            if frame.is_empty() {
                if byte == 0x55 {
                    frame.push(byte);
                }
            } else if frame.len() == 1 {
                if byte == 0x53 {
                    frame.push(byte);
                }
            } else {
                frame.push(byte);

                if frame.len() == 22 {
                    (yaw,gy,gz) = parse_frame(&frame);
                    if start_yaw == None {
                        start_yaw = Some(yaw)
                    }
                    let mut yaw_offset = yaw - start_yaw.unwrap();
                    if yaw_offset > 180.0  {yaw_offset -= 360.0}
                    if yaw_offset < -180.0 {yaw_offset += 360.0}
                    println!("Yaw: {:.2} \tAngular velocity -> y: {:.2}, z: {:.2}", yaw_offset, gy, gz);
                    frame.clear();
                }
            }
        }
    }
}

fn parse_frame(frame: &[u8]) -> (f32, f32, f32) {
    let yaw = i16::from_le_bytes([frame[6],  frame[7] ]) as f32 / 32768.0 * 180.0;
    let gy  = i16::from_le_bytes([frame[15], frame[16]]) as f32 / 32768.0 * 180.0;
    let gz  = i16::from_le_bytes([frame[17], frame[18]]) as f32 / 32768.0 * 180.0;

    (yaw, gy, gz)
}