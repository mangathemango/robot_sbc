use std::time::Duration;

use crate::control::{actions::general::{RuntimeSequence, Sequence, WaitFor}, routines::{navigation::{move_back_to_start, move_to_qr}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(wait_for_qr())
        .then(set_oled_display_text_qr())
        // .then(set_oled_display_text_stopq())
}