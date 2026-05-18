use crate::{
    ROBOT, control::actions::{
        extend_arm::{ExtendArm, RetractArm},
        general::{RuntimeSequence, Sequence},
        lift_arm::{LiftArm, LowerArm},
        rotate_arm::RotateArm,
        rotate_claw::RotateClaw,
    }, devices::maixcam::circle::MaixcamCircleColor
};


pub fn pick_up_all_materials_from_ground_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.get_qr_state().color_queue_2;
        
        let mut sequence = Sequence::new("Placing materials on the ground (Second sequence)");
        if let Some(queue) = queue {
            queue.into_iter().for_each(|color| {
                sequence.enqueue(grab_material_from_ground(color));
                sequence.enqueue(place_material_into_storage(color));
            });
        }
        sequence
    })
}

pub fn place_all_materials_on_ground_2() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.get_qr_state().color_queue_2;
        
        let mut sequence = Sequence::new("Placing all materials on the ground (Second sequence)");
        if let Some(queue) = queue {
            queue.into_iter().for_each(|color| {
                sequence.enqueue(grab_material_from_storage(color));
                sequence.enqueue(place_material_ground(color));
            });
        }
        sequence
    })
}

pub fn place_all_materials_stacked() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.get_qr_state().color_queue_2;
        
        let mut sequence = Sequence::new("Stacking all materials");
        if let Some(queue) = queue {
            queue.into_iter().for_each(|color| {
                sequence.enqueue(grab_material_from_storage(color));
                sequence.enqueue(place_material_stacked(color));
            });
        }
        sequence
    })
}



pub fn pick_up_all_materials_from_ground_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.get_qr_state().color_queue_1;
        
        let mut sequence = Sequence::new("Placing materials on the ground (First sequence)");
        if let Some(queue) = queue {
            queue.into_iter().for_each(|color| {
                sequence.enqueue(grab_material_from_ground(color));
                sequence.enqueue(place_material_into_storage(color));
            });
        }
        sequence
    })
}

pub fn place_all_materials_on_ground_1() -> RuntimeSequence {
    RuntimeSequence::new(|| {
        let queue = ROBOT.get_qr_state().color_queue_1;
        
        let mut sequence = Sequence::new("Placing materials on the ground (First sequence)");
        if let Some(queue) = queue {
            queue.into_iter().for_each(|color| {
                sequence.enqueue(grab_material_from_storage(color));
                sequence.enqueue(place_material_ground(color));
            });
        }
        sequence
    })
}

fn grab_material_from_ground(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Grabbing {:?} material from ground", color).as_str())
        .then(LiftArm::up())
        .then(RotateClaw::open())
        .then(RotateArm::to_placement(color))
        .then(ExtendArm::to_placement(color))
        .then(LowerArm::to_ground())
        .then(RotateClaw::close())
        .then(LiftArm::up())
}

fn grab_material_from_storage(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Grabbing {:?} material from storage", color).as_str())
        .then(LiftArm::up())
        .then(RetractArm::back())
        .then(RotateClaw::open())
        .then(RotateArm::to_storage(color))
        .then(LowerArm::to_storage())
        .then(ExtendArm::to_storage(color))
        .then(RotateClaw::close())
        .then(LiftArm::up())
}

fn place_material_ground(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Placing {:?} material on the ground", color).as_str())
        .then(LiftArm::up())
        .then(ExtendArm::to_placement(color))
        .then(RotateClaw::close())
        .then(RotateArm::to_placement(color))
        .then(LowerArm::to_ground())
        .then(RotateClaw::open())
        .then(LiftArm::up())
        .then(RetractArm::back())
}

fn place_material_stacked(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Stacking {:?} material", color).as_str())
        .then(LiftArm::up())
        .then(ExtendArm::to_placement(color))
        .then(RotateClaw::close())
        .then(RotateArm::to_placement(color))
        .then(LowerArm::to_stacked())
        .then(RotateClaw::open())
        .then(LiftArm::up())
        .then(RetractArm::back())
}

fn place_material_into_storage(color: MaixcamCircleColor) -> Sequence {
    Sequence::new(format!("Placing {:?} material into storage", color).as_str())
        .then(RotateClaw::close())
        .then(LiftArm::up())
        .then(RotateArm::to_storage(color))
        .then(ExtendArm::to_storage(color))
        .then(LowerArm::to_storage())
        .then(RotateClaw::open())
        .then(RetractArm::back())
        .then(LiftArm::up())
}
