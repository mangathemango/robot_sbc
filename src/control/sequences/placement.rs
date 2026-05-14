use crate::control::actions::extend_arm::{ArmExtendPreset, ExtendArm};
use crate::control::actions::lift_arm::{ArmLiftPreset, LiftArm, LowerArm};
use crate::control::actions::rotate_arm::{ArmRotationPreset, RotateArm};
use crate::control::actions::rotate_claw::RotateClaw;
use crate::control::sequences::Sequence;

fn grab_material(
    source_rotation: ArmRotationPreset,
    source_height: ArmLiftPreset,
    source_extension: ArmExtendPreset,
) -> Sequence {
    Sequence::new(
        format!(
            "Grabing material from rotation {:?} and height {:?}",
            source_rotation, source_height
        )
        .as_str(),
    )
    .then(LiftArm::up())
    .then(ExtendArm::back())
    .then(RotateArm::to_preset(source_rotation))
    .then(RotateClaw::open())
    .then(LowerArm::to_preset(source_height))
    .then(ExtendArm::to_preset(source_extension))
    .then(RotateClaw::close())
    .then(LiftArm::up())
    .then(ExtendArm::back())
}

fn place_material(
    target_rotation: ArmRotationPreset,
    target_height: ArmLiftPreset,
    target_extension: ArmExtendPreset,
) -> Sequence {
    Sequence::new(
        format!(
            "Placing material at angle {:?} and height {:?}",
            target_rotation, target_height
        )
        .as_str(),
    )
    .then(LiftArm::up())
    .then(ExtendArm::back())
    .then(RotateArm::to_preset(target_rotation))
    .then(ExtendArm::to_preset(target_extension))
    .then(LowerArm::to_preset(target_height))
    .then(RotateClaw::open())
    .then(LiftArm::up())
}
