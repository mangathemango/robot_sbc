use crate::ROBOT;
use crate::control::actions::general::OneShot;
use crate::control::actions::r#move::Move;
use crate::control::actions::rotate_arm::RotateArm;
use crate::control::landmark::Landmark;
use crate::control::motion::MotionPolicyPreset;
use crate::control::sequences::Sequence;

pub fn move_to_qr() -> Sequence {
    Sequence::new("Moving to Qr Zone")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::QrZone))
}

pub fn move_from_qr_to_source_zone() -> Sequence {
    Sequence::new("Moving from Qr zone to Source Zone")
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::right())
}

pub fn move_to_temporary_storage_zone() -> Sequence {
    Sequence::new("Moving to Temporary storage zone")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::CentralRightCrossing).policy(MotionPolicyPreset::Aggressive))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::left())
}

pub fn move_to_final_processing_zone() -> Sequence {
    Sequence::new("Moving to Final processing zone")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperLeftTurn).policy(MotionPolicyPreset::Aggressive))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::right())
}

pub fn move_from_final_processing_zone_to_source() -> Sequence {
    Sequence::new("Moving from Final processing zone to source zone")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperRightTurn).policy(MotionPolicyPreset::Aggressive))
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::right())
}

pub fn move_back_to_start() -> Sequence {
    Sequence::new("Moving back to start zone")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperRightTurn).policy(MotionPolicyPreset::Aggressive))
        .then(Move::to(Landmark::Start))
}

pub fn set_current_landmark(landmark: Landmark) -> OneShot {
    OneShot::new(move || {
        ROBOT.lock_odometry_state().pose = landmark.pose();
    })
}
