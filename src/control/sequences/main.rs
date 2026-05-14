use std::time::Duration;

use crate::control::{actions::general::WaitFor, sequences::{Sequence, navigation::*, placement::*}};


pub fn main_sequence() -> Sequence {
    Sequence::new("Main Sequence")
        .then(flag_1())
        .then(flag_2())
        .then(flag_3())
        .then(flag_4())
        .then(flag_5())
        .then(flag_6())
        .then(flag_7())
        .then(flag_8())
}