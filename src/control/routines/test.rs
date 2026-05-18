use std::time::Duration;

use crate::{ROBOT, control::{actions::{general::{OneShot, RuntimeSequence, Sequence, WaitFor}, rotate_claw::RotateClaw}, routines::{navigation::{move_back_to_start, move_to_qr}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(set_oled_display_text_start())
        .then(OneShot::new(|| {
            let mut qr_state = ROBOT.lock_qr_state();
            qr_state.reset();
        }))
        .then(RotateClaw::close())
        .then(WaitFor::new(Duration::from_millis(1000)))
        .then(RotateClaw::open())
        .then(set_oled_display_text_stop())
        // .then(set_oled_display_text_stopq())
}