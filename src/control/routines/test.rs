use crate::control::{actions::general::{RuntimeSequence, Sequence}, routines::navigation::{move_back_to_start, move_to_qr}};

pub fn test_sequence() -> Sequence {
    Sequence::new("Test Sequence")
        .then(move_to_qr())
        .then(move_back_to_start())
        .then(RuntimeSequence::new(|| test_sequence()))
}