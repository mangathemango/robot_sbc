use std::time::{Duration, Instant};

use glam::Vec2;

use crate::{ROBOT, control::{actions::{calibrate_source::CalibrateSource, general::{OneShot, RuntimeSequence, Sequence, WaitFor}, r#move::Move, rotate_claw::RotateClaw}, landmark::Landmark, motion::{MotionPolicy, MotionPolicyPreset}, routines::{calibration::{calibrate_at_final_processing_zone, calibrate_at_source_zone}, navigation::{move_back_to_start, move_to_qr, set_current_landmark}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}, math::{PidController, Pose}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")

        .then(beep())
        .then(test_gyro())

        // .then(WaitFor::new(Duration::from_millis(1000)))
}   

pub fn test_gyro() -> Sequence {
    Sequence::new("Testing gyro fr")
        .then(set_current_landmark(Landmark::Start))
        .then(Move::to(Landmark::Start).policy(MotionPolicyPreset::Custom(
            MotionPolicy {
                linear_pid: PidController::new(0.0, 0.0, 0.0, 0.0, 0.0),
                angular_pid: PidController::new(-0.01, 0.0, 0.0, 0.1, 0.0),
                settle_time: Duration::from_millis(10000), 
            }
        )))
}