use evdev::{Device, EventSummary, KeyCode, EvdevEnum};

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
        <dyn EvdevEnum>::KEY_1 => '1',
        <dyn EvdevEnum>::KEY_2 => '2',
        <dyn EvdevEnum>::KEY_3 => '3',
        <dyn EvdevEnum>::KEY_4 => '4',
        <dyn EvdevEnum>::KEY_5 => '5',
        <dyn EvdevEnum>::KEY_6 => '6',
        <dyn EvdevEnum>::KEY_7 => '7',
        <dyn EvdevEnum>::KEY_8 => '8',
        <dyn EvdevEnum>::KEY_9 => '9',
        <dyn EvdevEnum>::KEY_0 => '0',
        <dyn EvdevEnum>::KPPLUS => '+',
        <dyn EvdevEnum>::KEY_MINUS => '-',
        <dyn EvdevEnum>::KEY_SLASH => '/',
        <dyn EvdevEnum>::KEY_DOT => '.',
        <dyn EvdevEnum>::KEY_SPACE => ' ',

        _ => return None,
    };

    // optional shift handling
    Some(if shift { c.to_ascii_uppercase() } else { c })
}