pub mod r#move;
pub mod rotate_arm;
pub mod rotate_claw;
pub mod lift_arm;
pub mod extend_arm;
pub mod calibrate_placement;
pub mod general;

use std::{fmt::{Debug, Display}, time::Duration};

pub trait Action: Display {
    fn start(&mut self) {}

    fn update(&mut self, dt: Duration) {}

    fn is_finished(&self) -> bool {true}

    fn stop(&mut self) {}

    fn current_action(&self) -> &dyn Action;
}
