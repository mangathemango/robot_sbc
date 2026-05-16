use crate::ROBOT;
use crate::control::actions::general::OneShot;
use crate::control::actions::r#move::Move;
use crate::control::actions::rotate_arm::RotateArm;
use crate::control::landmark::Landmark;
use crate::control::sequences::Sequence;
use crate::math::Pose;

pub fn move_to_qr() -> Sequence {
    Sequence::new("Flag 1")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::QrZone))
}

pub fn move_from_qr_to_source_zone() -> Sequence {
    Sequence::new("Flag 2")
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::right())
}

pub fn move_to_temporary_storage_zone() -> Sequence {
    Sequence::new("Flag 3")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::CentralRightCrossing))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::left())
}

pub fn move_to_final_processing_zone() -> Sequence {
    Sequence::new("Flag 4")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperLeftTurn))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::right())
}

pub fn move_from_final_processing_zone_to_source() -> Sequence {
    Sequence::new("Flag 5")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperRightTurn))
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::right())
}

pub fn flag_6() -> Sequence {
    Sequence::new("Flag 6")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::CentralRightCrossing))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::left())
}

pub fn flag_7() -> Sequence {
    Sequence::new("Flag 7")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperLeftTurn))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::right())
}

pub fn move_back_to_start() -> Sequence {
    Sequence::new("Flag 8")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperRightTurn))
        .then(Move::to(Landmark::Start))
}

pub fn set_current_landmark(landmark: Landmark) -> OneShot {
    OneShot::new(move || {
        ROBOT.lock_odometry_state().pose = landmark.pose();
    })
}
