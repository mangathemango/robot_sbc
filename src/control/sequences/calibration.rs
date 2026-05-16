use crate::control::{
    actions::{
        calibrate_placement::CalibratePlacement, calibrate_source::CalibrateSource,
        rotate_arm::ArmRotationPreset,
    },
    landmark::Landmark,
    sequences::{Sequence, navigation::set_current_landmark},
};

pub fn calibrate_at_source_zone() -> Sequence {
    Sequence::new("Calibrating at source zone")
        .then(CalibrateSource::new())
        .then(set_current_landmark(Landmark::SourceZone))
}

pub fn calibrate_at_temporary_storage_zone() -> Sequence {
    Sequence::new("Calibrating at temporary storage zone")
        .then(CalibratePlacement::ground().with_arm_rotation(ArmRotationPreset::Left))
        .then(set_current_landmark(Landmark::TemporaryStorageZone))
}

pub fn calibrate_at_final_processing_zone() -> Sequence {
    Sequence::new("Calibrating at final processing zone (first round)")
        .then(CalibratePlacement::ground().with_arm_rotation(ArmRotationPreset::Right))
        .then(set_current_landmark(Landmark::FinalProcessingZone))
}

pub fn calibrate_at_final_processing_zone_stacked() -> Sequence {
    Sequence::new("Calibrating at final processing zone (second round)")
        .then(CalibratePlacement::stack().with_arm_rotation(ArmRotationPreset::Right))
        .then(set_current_landmark(Landmark::FinalProcessingZone))
}
