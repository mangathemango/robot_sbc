use serialport::SerialPort;
use std::io::Read;
use std::time::Duration;

fn main() {
    let mut port = serialport::new("COM8", 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut buffer = [0u8; 1];
    let mut frame: Vec<u8> = Vec::new();

    loop {
        if let Ok(_) = port.read_exact(&mut buffer) {
            let byte = buffer[0];

            if frame.is_empty() {
                if byte == 0x55 {
                    frame.push(byte);
                }
            } else {
                frame.push(byte);

                if frame.len() == 11 {
                    parse_frame(&frame);
                    frame.clear();
                }
            }
        }
    }
}

fn parse_frame(frame: &[u8]) {
    let frame_type = frame[1];

    match frame_type {
        0x53 => {
            let yaw   = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;

            print!("Yaw: {:.2} \t", yaw);
        }

        0x52 => {
            let gy = i16::from_le_bytes([frame[4], frame[5]]) as f32 / 32768.0 * 180.0;
            let gz = i16::from_le_bytes([frame[6], frame[7]]) as f32 / 32768.0 * 180.0;

            println!("Angular velocity -> y: {:.2}, z: {:.2}", gy, gz);
        }

        _ => {
            // ignore others for now
        }
    }
}