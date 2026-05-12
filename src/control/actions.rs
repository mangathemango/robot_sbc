pub mod r#move;
pub mod rotate_arm;

use crate::control::ControllerState;
use std::{fmt::Debug, time::Duration};

pub trait Action: Debug {
    fn start(&mut self, state: &mut ControllerState);

    fn update(&mut self, state: &mut ControllerState, dt: Duration);

    fn is_finished(&self) -> bool;

    fn stop(&mut self, state: &mut ControllerState);
}
