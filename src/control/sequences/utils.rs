use crate::{
    ROBOT,
    control::{
        actions::general::{OneShot, WaitUntil},
        sequences::Sequence,
    },
    debug::widgets::qr,
    math::Pose,
};

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
        let qr_text = ROBOT.qr_state.load().code.clone();
        ROBOT.get_stm32_controller().set_display_text(qr_text)
    })
}

pub fn beep() -> OneShot {
    OneShot::new(|| ROBOT.get_stm32_controller().beep())
}
