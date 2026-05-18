use std::time::Duration;

use crate::control::{actions::general::{RuntimeSequence, Sequence, WaitFor}, routines::{navigation::{move_back_to_start, move_to_qr}, utils::beep}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(beep())
        .then(WaitFor::new(Duration::from_millis(1000)))
        .then(beep())
        .then(WaitFor::new(Duration::from_millis(1000)))
        .then(beep())
        .then(WaitFor::new(Duration::from_millis(1000)))
}