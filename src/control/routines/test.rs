use std::time::Duration;

use crate::{ROBOT, control::{actions::general::{OneShot, RuntimeSequence, Sequence, WaitFor}, routines::{navigation::{move_back_to_start, move_to_qr}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(set_oled_display_text_start())
        .then(OneShot::new(|| {
            ROBOT.lock_qr_state().code = None;
        }))
        .then(wait_for_qr())
        .then(set_oled_display_text_qr())
        // .then(set_oled_display_text_stopq())
}