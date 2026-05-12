pub mod main;

use std::{collections::VecDeque, time::Duration};
use crate::control::actions::Action;

#[derive(Debug, Default)]
pub struct Sequence {
    pub action_queue: VecDeque<Box<dyn Action>>,
    pub current_action: Option<Box<dyn Action>>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn enqueue<A>(&mut self, action: A) -> &mut Self
    where
        A: Action + 'static,
    {
        self.action_queue.push_back(Box::new(action));
        self
    }

    pub fn then<A>(mut self, action: A) -> Self
    where
        A: Action + 'static,
    {
        self.action_queue.push_back(Box::new(action));
        self
    }
}

impl Action for Sequence {
    fn start(&mut self) {

    }

    fn update(&mut self, dt: Duration) {
        match &mut self.current_action {
            None => {
                self.current_action = self.action_queue.pop_front();

                if let Some(action) = &mut self.current_action {
                    action.start();
                }
            }
            Some(action) => {
                action.update(dt);
                if action.is_finished() {
                    action.stop();
                    self.current_action = None;
                }
            }
        }

    }

    fn is_finished(&self) -> bool {
        self.action_queue.is_empty() && self.current_action.is_none()
    }

    fn stop(&mut self) {
        if let Some(mut action) = self.current_action.take() {
            action.stop();
        }
        self.action_queue.clear();
    }

    fn current_action(&self) -> &dyn Action {
        if let Some(action) = &self.current_action {
            action.current_action()
        } else {
            self
        }
    }
}