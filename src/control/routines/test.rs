use std::{f32::consts::{FRAC_PI_2, PI}, time::{Duration, Instant}};

use glam::Vec2;

use crate::{ROBOT, control::{actions::{calibrate_source::CalibrateSource, general::{OneShot, RuntimeSequence, Sequence, WaitFor}, r#move::Move, rotate_claw::RotateClaw}, landmark::Landmark, motion::{MotionPolicy, MotionPolicyPreset}, routines::{calibration::{calibrate_at_final_processing_zone, calibrate_at_source_zone}, navigation::{move_back_to_start, move_to_qr, set_current_landmark}, utils::{beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop, wait_for_qr}}}, math::{PidController, Pose}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(beep())
        .then(set_oled_display_text_start())
        .then(set_current_landmark(Landmark::Start))
        .then(Move::to(Landmark::QrZone))
        .then(set_oled_display_text_stop())

        // .then(WaitFor::new(Duration::from_millis(1000)))
}   

pub fn test_movement() -> Sequence {
    Sequence::new("Movement")
        .then(OneShot::new(|| {
            ROBOT.stm32_controller().set_wheel_velocities([100,100,100,100]);
        }))
        .then(WaitFor::new(Duration::from_millis(500)))
        .then(OneShot::new(|| {
            ROBOT.stm32_controller().set_wheel_velocities([0,0,0,0]);
        }))
}

pub fn test_gyro() -> Sequence {
    let policy = MotionPolicy {
        linear_pid: PidController::new(0.0, 0.0, 0.0, 10.0, 0.0),
        angular_pid: PidController::new(-0.008, -0.0005, -0.0005, 0.01, 1.0),
        settle_time: Duration::from_millis(1000), 
    };
    Sequence::new("Testing gyro fr")
        .then(set_current_landmark(Landmark::Custom(Pose {
            position: Vec2::ZERO,
            rotation: 0.0,
        })))
        .then(Move::to(Landmark::Custom(Pose {position: Vec2::ZERO,rotation: 0.0,}))
            .policy(MotionPolicyPreset::Custom(policy)))
        .then(Move::to(Landmark::Custom(Pose {position: Vec2::ZERO,rotation: FRAC_PI_2,}))
            .policy(MotionPolicyPreset::Custom(policy)))
        .then(Move::to(Landmark::Custom(Pose {position: Vec2::ZERO,rotation: PI,}))
            .policy(MotionPolicyPreset::Custom(policy)))
        .then(Move::to(Landmark::Custom(Pose {position: Vec2::ZERO,rotation: -FRAC_PI_2,}))
            .policy(MotionPolicyPreset::Custom(policy)))
        .then(Move::to(Landmark::Custom(Pose {position: Vec2::ZERO,rotation: 0.0,}))
            .policy(MotionPolicyPreset::Custom(policy)))
}