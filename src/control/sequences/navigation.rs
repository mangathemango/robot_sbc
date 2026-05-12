use crate::control::actions::r#move::Move;
use crate::control::actions::rotate_arm::RotateArm;
use crate::control::landmark::Landmark;
use crate::control::sequences::Sequence;

pub fn flag_1() -> Sequence {
    Sequence::new("Flag 1")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::QrZone))
}

pub fn flag_2() -> Sequence {
    Sequence::new("Flag 2")
        .then(Move::to(Landmark::SourceZone))
        .then(RotateArm::right())
}

pub fn flag_3() -> Sequence {
    Sequence::new("Flag 3")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::CentralRightCrossing))
        .then(Move::to(Landmark::TemporaryStorageZone))
        .then(RotateArm::left())
}

pub fn flag_4() -> Sequence {
    Sequence::new("Flag 4")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperLeftTurn))
        .then(Move::to(Landmark::FinalProcessingZone))
        .then(RotateArm::right())
}

pub fn flag_5() -> Sequence {
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

pub fn flag_8() -> Sequence {
    Sequence::new("Flag 8")
        .then(RotateArm::middle())
        .then(Move::to(Landmark::UpperRightTurn))
        .then(Move::to(Landmark::Start))
}
