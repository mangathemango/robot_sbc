use std::time::{Duration, Instant};

use glam::Vec2;

use crate::{ROBOT, control::{actions::{calibrate_source::CalibrateSource, general::{OneShot, RuntimeSequence, Sequence, WaitFor}, rotate_claw::RotateClaw}, landmark::Landmark, routines::{calibration::{calibrate_at_final_processing_zone, calibrate_at_source_zone}, navigation::{move_back_to_start, move_to_qr, set_current_landmark}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}, math::Pose};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(beep())
}   