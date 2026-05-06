use evdev::{Device, EventSummary, KeyCode, EvdevEnum};

pub fn try_read() -> Result<(), String> {
    let path = "/dev/input/by-id/usb-USBKey_Chip_USBKey_Module_202730041341-event-kbd";
    let mut device = Device::open(path)
        .map_err(|e| format!("{}", e))?;

    let mut buffer = String::new();

    loop {
        for ev in device.fetch_events().map_err(|e| format!("{}", e))? {
            match ev.destructure() {
                EventSummary::Key(_, keycode, value) => {
                    if value == 1 {
                        match keycode {
                            evdev::KeyCode::KEY_ENTER => {
                                println!("QR: {:?}", buffer);
                                buffer.clear();
                            }
                            _ => {
                                if let Some(char) = keycode_to_ascii(keycode, false) {
                                    buffer.push(char);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn keycode_to_ascii(key: KeyCode, shift: bool) -> Option<char> {

    let c = match key {
        KeyCode::KEY_1 => '1',
        KeyCode::KEY_2 => '2',
        KeyCode::KEY_3 => '3',
        KeyCode::KEY_4 => '4',
        KeyCode::KEY_5 => '5',
        KeyCode::KEY_6 => '6',
        KeyCode::KEY_7 => '7',
        KeyCode::KEY_8 => '8',
        KeyCode::KEY_9 => '9',
        KeyCode::KEY_0 => '0',
        KeyCode::KEY_KPPLUS => '+',
        KeyCode::KEY_MINUS => '-',
        KeyCode::KEY_SLASH => '/',
        KeyCode::KEY_DOT => '.',
        KeyCode::KEY_SPACE => ' ',

        _ => return None,
    };

    // optional shift handling
    Some(if shift { c.to_ascii_uppercase() } else { c })
}