use std::collections::VecDeque;
use crate::control::actions::Action;

pub struct Sequence {
    pub action_queue: VecDeque<Box<dyn Action>>,
    pub current_action: Option<Box<dyn Action>>,
}