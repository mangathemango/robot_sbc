use evdev::{Device, EventSummary, KeyCode};

pub fn try_read() -> Result<(), String> {
    let path = "/dev/input/by-id/usb-USBKey_Chip_USBKey_Module_202730041341-event-kbd";
    let mut device = Device::open(path)
        .map_err(|e| format!("{}", e))?;

    let mut buffer = Vec::new();

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
                                buffer.push(keycode_to_ascii(keycode, false));
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
        Keycode::KEY_1 => '1',
        Keycode::KEY_2 => '2',
        Keycode::KEY_3 => '3',
        Keycode::KEY_4 => '4',
        Keycode::KEY_5 => '5',
        Keycode::KEY_6 => '6',
        Keycode::KEY_7 => '7',
        Keycode::KEY_8 => '8',
        Keycode::KEY_9 => '9',
        Keycode::KEY_0 => '0',
        KeyCode::KPPLUS => '+',
        Keycode::KEY_MINUS => '-',
        Keycode::KEY_SLASH => '/',
        Keycode::KEY_DOT => '.',
        Keycode::KEY_SPACE => ' ',

        _ => return None,
    };

    // optional shift handling
    Some(if shift { c.to_ascii_uppercase() } else { c })
}