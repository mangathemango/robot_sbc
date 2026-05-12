use crate::control::actions::r#move::Move;
use crate::control::actions::rotate_arm::RotateArm;
use crate::control::landmark::Landmark;
use crate::control::sequences::Sequence;
use crate::control::states::yaw_servo::ArmPosition;

pub fn flag_1() -> Sequence {
    Sequence::new()
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::QrZone))
}

pub fn flag_2() -> Sequence {
    Sequence::new()
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::to(ArmPosition::Right))
}

pub fn flag_3() -> Sequence {
    Sequence::new()
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::CentralRightCrossing))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::to(ArmPosition::Left))
}

pub fn flag_4() -> Sequence {
    Sequence::new()        
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::UpperLeftTurn))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::to(ArmPosition::Right))
}

pub fn flag_5() -> Sequence {
    Sequence::new()        
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::UpperRightTurn))
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::to(ArmPosition::Right))
}

pub fn flag_6() -> Sequence {
    Sequence::new()
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::CentralRightCrossing))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::to(ArmPosition::Left))
}

pub fn flag_7() -> Sequence {
    Sequence::new()
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::UpperLeftTurn))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::to(ArmPosition::Right))
}

pub fn flag_8() -> Sequence {
    Sequence::new()
        .then(RotateArm::to(ArmPosition::Middle))
        .then(Move::to(Landmark::UpperRightTurn))
        .then(Move::to(Landmark::Start))
}