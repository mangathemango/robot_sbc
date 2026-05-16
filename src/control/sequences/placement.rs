use std::time::Duration;

use crate::ROBOT;
use crate::control::actions::extend_arm::{ArmExtendPreset, ExtendArm};
use crate::control::actions::general::WaitFor;
use crate::control::actions::lift_arm::{ArmLiftPreset, LiftArm, LowerArm};
use crate::control::actions::rotate_arm::{ArmRotationPreset, RotateArm};
use crate::control::actions::rotate_claw::RotateClaw;
use crate::control::sequences::{RuntimeSequence, Sequence};
use crate::devices::maixcam::circle::MaixcamCircleColor;

pub fn place_material_at_temporary_storage_zone_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_1 {
            place_material_at_temporary_storage_zone(color_queue)
        } else {
            Sequence::new("Skipping place material...")
                .then(WaitFor::new(Duration::from_millis(3000)))
        }
    })
}

pub fn pick_up_material_from_temporary_storage_zone_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_1 {
            pick_up_material_from_temporary_storage_zone(color_queue)
        } else {
            Sequence::new("Skipping pickup material...")
                .then(WaitFor::new(Duration::from_millis(3000)))
        }
    })
}

pub fn place_material_at_temporary_storage_zone_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_2 {
            place_material_at_temporary_storage_zone(color_queue)
        } else {
            Sequence::new("Skipping place material...")
                .then(WaitFor::new(Duration::from_millis(3000)))
        }
    })
}
pub fn pick_up_material_from_temporary_storage_zone_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_2 {
            pick_up_material_from_temporary_storage_zone(color_queue)
        } else {
            Sequence::new("Skipping pickup material...")
                .then(WaitFor::new(Duration::from_millis(3000)))
        }
    })
}

pub fn place_material_at_final_processing_zone() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let mut sequence = Sequence::new("Placing sequence at temporary storage zone");

        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_1 {
            if color_queue.len() != 3 {
                return sequence;
            }

            let storage_order = [
                (ArmRotationPreset::LeftStorage, color_queue[0]),
                (ArmRotationPreset::MiddleStorage, color_queue[1]),
                (ArmRotationPreset::RightStorage, color_queue[2]),
            ];

            for (storage, color) in storage_order {
                sequence
                    .enqueue(grab_material(
                        storage,
                        ArmLiftPreset::Storage,
                        ArmExtendPreset::Storage,
                    ))
                    .enqueue(place_material(
                        ArmRotationPreset::LeftPlacement(color),
                        ArmLiftPreset::Ground,
                        color.placement_arm_extension(),
                    ));
            }
        }

        sequence
    })
}

pub fn place_material_at_final_processing_zone_stacked() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let mut sequence = Sequence::new("Placing sequence at temporary storage zone");

        if let Some(color_queue) = ROBOT.get_qr_state().color_queue_1 {
            if color_queue.len() != 3 {
                return sequence;
            }

            let storage_order = [
                (ArmRotationPreset::LeftStorage, color_queue[0]),
                (ArmRotationPreset::MiddleStorage, color_queue[1]),
                (ArmRotationPreset::RightStorage, color_queue[2]),
            ];

            for (storage, color) in storage_order {
                sequence
                    .enqueue(grab_material(
                        storage,
                        ArmLiftPreset::Storage,
                        ArmExtendPreset::Storage,
                    ))
                    .enqueue(place_material(
                        ArmRotationPreset::LeftPlacement(color),
                        ArmLiftPreset::Stack,
                        color.placement_arm_extension(),
                    ));
            }
        }

        sequence
    })
}

fn pick_up_material_from_temporary_storage_zone(color_queue: Vec<MaixcamCircleColor>) -> Sequence {
    let mut sequence = Sequence::new("Placing sequence at temporary storage zone");

    if color_queue.len() != 3 {
        return sequence;
    }

    let storage_order = [
        (ArmRotationPreset::LeftStorage, color_queue[0]),
        (ArmRotationPreset::MiddleStorage, color_queue[1]),
        (ArmRotationPreset::RightStorage, color_queue[2]),
    ];

    for (storage, color) in storage_order {
        sequence
            .enqueue(grab_material(
                ArmRotationPreset::LeftPlacement(color),
                ArmLiftPreset::Ground,
                color.placement_arm_extension(),
            ))
            .enqueue(place_material(
                storage,
                ArmLiftPreset::Storage,
                ArmExtendPreset::Storage,
            ));
    }

    sequence
}

fn place_material_at_temporary_storage_zone(color_queue: Vec<MaixcamCircleColor>) -> Sequence {
    let mut sequence = Sequence::new("Placing sequence at temporary storage zone");

    if color_queue.len() != 3 {
        return sequence;
    }

    let storage_order = [
        (ArmRotationPreset::LeftStorage, color_queue[0]),
        (ArmRotationPreset::MiddleStorage, color_queue[1]),
        (ArmRotationPreset::RightStorage, color_queue[2]),
    ];

    for (storage, color) in storage_order {
        sequence
            .enqueue(grab_material(
                storage,
                ArmLiftPreset::Storage,
                ArmExtendPreset::Storage,
            ))
            .enqueue(place_material(
                ArmRotationPreset::LeftPlacement(color),
                ArmLiftPreset::Ground,
                color.placement_arm_extension(),
            ));
    }

    sequence
}

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
