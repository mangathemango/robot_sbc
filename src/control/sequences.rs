pub mod main;
pub mod navigation;
pub mod placement;

use core::fmt;
use std::{collections::VecDeque, fmt::Display, time::Duration};
use crate::control::actions::Action;

#[derive(Debug, Default)]
pub struct Sequence {
    pub name: String,
    pub action_queue: VecDeque<Box<dyn Action>>,
    pub current_action: Option<Box<dyn Action>>,
}

impl Sequence {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
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

    fn current_action_string(&self) -> String {
        if let Some(action) = self.current_action.as_ref() {
            format!("{}", action)
        } else {
            "Empty".into()
        }
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

impl Display for Sequence {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: \n{}", 
            self.name, self.current_action_string())
    }
}