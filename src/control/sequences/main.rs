use crate::control::{
    sequences::{
        Sequence,
        calibration::{
            calibrate_at_final_processing_zone_1, calibrate_at_final_processing_zone_2,
            calibrate_at_source_zone, calibrate_at_temporary_storage_zone,
        },
        navigation::*,
    },
};

pub fn main_sequence() -> Sequence {
    Sequence::new("Main Sequence")
        .then(move_to_qr())
        .then(move_from_qr_to_source_zone())
        .then(calibrate_at_source_zone())
        .then(move_to_temporary_storage_zone())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone_1())
        .then(move_from_final_processing_zone_to_source())
        .then(calibrate_at_source_zone())
        .then(move_to_temporary_storage_zone())
        .then(calibrate_at_temporary_storage_zone())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone_2())
        .then(move_back_to_start())
}
