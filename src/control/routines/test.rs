use std::time::Duration;

use crate::{ROBOT, control::{actions::{calibrate_source::CalibrateSource, general::{OneShot, RuntimeSequence, Sequence, WaitFor}, rotate_claw::RotateClaw}, routines::{calibration::calibrate_at_source_zone, navigation::{move_back_to_start, move_to_qr}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(calibrate_at_source_zone())
        // .then(set_oled_display_text_stopq())
}