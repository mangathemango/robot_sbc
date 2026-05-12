pub mod r#move;
pub mod rotate_arm;
pub mod rotate_claw;
pub mod lift_arm;
pub mod extend_arm;

use std::{fmt::{Debug, Display}, time::Duration};

pub trait Action: Debug + Display {
    fn start(&mut self);

    fn update(&mut self, dt: Duration);

    fn is_finished(&self) -> bool;

    fn stop(&mut self);

    fn current_action(&self) -> &dyn Action;
    fn name(&self) -> String;
}
