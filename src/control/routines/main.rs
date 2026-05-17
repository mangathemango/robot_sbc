use crate::control::{actions::general::Sequence, routines::{
    calibration::{
        calibrate_at_final_processing_zone, calibrate_at_final_processing_zone_stacked,
        calibrate_at_source_zone, calibrate_at_temporary_storage_zone,
    },
    navigation::*,
    placement::{

    },
    utils::{
        beep, set_oled_display_text_qr, set_oled_display_text_start, set_oled_display_text_stop,
        wait_for_qr,
    },
}};

pub fn main_sequence() -> Sequence {
    Sequence::new("Main Sequence")
        .then(beep())
        .then(set_oled_display_text_start())
        .then(move_to_qr())
        .then(wait_for_qr())
        .then(set_oled_display_text_qr())
        .then(move_from_qr_to_source_zone())
        .then(calibrate_at_source_zone())
        .then(move_to_temporary_storage_zone())
        .then(calibrate_at_temporary_storage_zone())
        .then(wait_for_qr())
        // .then(place_material_at_temporary_storage_zone_1())
        // .then(pick_up_material_from_temporary_storage_zone_1())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone())
        // .then(place_material_at_final_processing_zone())
        .then(move_from_final_processing_zone_to_source())
        .then(calibrate_at_source_zone())
        .then(move_to_temporary_storage_zone())
        .then(calibrate_at_temporary_storage_zone())
        // .then(place_material_at_temporary_storage_zone_2())
        // .then(pick_up_material_from_temporary_storage_zone_2())
        .then(move_to_final_processing_zone())
        .then(calibrate_at_final_processing_zone_stacked())
        // .then(place_material_at_final_processing_zone_stacked())
        .then(move_back_to_start())
        .then(set_oled_display_text_stop())
}
