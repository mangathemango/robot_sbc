use crate::{
    ROBOT,
    control::{
        actions::general::{OneShot, WaitUntil},
    },
};

pub fn wait_for_qr() -> WaitUntil {
    WaitUntil::new("Qr is scanned", || {
        ROBOT.get_qr_state().color_queue_1.is_some()
        && ROBOT.get_qr_state().color_queue_2.is_some()
    })
}

pub fn set_oled_display_text_start() -> OneShot {
    OneShot::new(|| {
        ROBOT
            .get_stm32_controller()
            .set_display_text("Started!".into())
    })
}

pub fn set_oled_display_text_stop() -> OneShot {
    OneShot::new(|| {
        ROBOT
            .get_stm32_controller()
            .set_display_text("Stopped".into())
    })
}

pub fn set_oled_display_text_qr() -> OneShot {
    OneShot::new(|| {
        let qr_text = ROBOT.get_qr_state().code.clone();
        if let Some(text) = qr_text {
            ROBOT.get_stm32_controller().set_display_text(text)
        }
    })
}

pub fn beep() -> OneShot {
    OneShot::new(|| ROBOT.get_stm32_controller().beep())
}
