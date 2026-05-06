use evdev::{Device, EventSummary};

pub fn try_read() {
    let path = "/dev/input/by-id/usb-USBKey_Chip_USBKey_Module_202730041341-event-kbd";
    let mut device = Device::open(path)?;

    let mut buffer = Vec::new();

    loop {
        for ev in device.fetch_events()? {
            match ev.destructure() {
                EventSummary::Key(_, keycode, value) => {
                    if value == 1 {
                        match keycode {
                            evdev::KeyCode::KEY_ENTER => {
                                println!("QR: {:?}", buffer);
                                buffer.clear();
                            }
                            _ => {
                                buffer.push(keycode);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}