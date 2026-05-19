use crate::control::actions::Action;
use std::{fmt::Display, time::Duration};
use core::fmt;
use std::collections::VecDeque;

#[derive(Default)]
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

    pub fn abort(&mut self) {
        if let Some(mut action) = self.current_action.take() {
            action.stop();
        }
        self.action_queue.clear();
        self.current_action = None;
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
}

impl Display for Sequence {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: \n{}", 
            self.name, self.current_action_string())
    }
}

pub struct RuntimeSequence
{
    generator: Box<dyn Fn() -> Sequence + Send + Sync>,
    generated: Sequence,
}

impl RuntimeSequence
{   
    pub fn new<F>(f: F) -> Self 
    where 
        F: Fn() -> Sequence + Send + Sync + 'static
    {
        Self {
            generator: Box::new(f),
            generated: Sequence::new("Unknown generated function")
        }
    }
}

impl Action for RuntimeSequence
{
    fn start(&mut self) {
        self.generated = (self.generator)()    
    }

    fn update(&mut self, dt: Duration) {
        self.generated.update(dt);
    }

    fn stop(&mut self) {
        self.generated.stop();
    }

    fn is_finished(&self) -> bool {
        self.generated.is_finished()
    }
}

impl Display for RuntimeSequence
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Runtime Generated) {}", self.generated)
    }
}

pub struct OneShot {
    f: Box<dyn FnMut() + Send + Sync>,
}

impl OneShot {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static + Send + Sync,
    {
        Self { f: Box::new(f) }
    }
}

impl Action for OneShot {
    fn start(&mut self) {
        (self.f)()
    }

    fn is_finished(&self) -> bool {
        true
    }
}

impl Display for OneShot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instant")
    }
}

#[derive(Debug, Default)]
pub struct WaitFor {
    wait_duration: Duration,
    elapsed: Duration,
}

impl WaitFor {
    pub fn new(wait_duration: Duration) -> Self {
        Self {
            wait_duration,
            elapsed: Duration::ZERO,
        }
    }
}

impl Action for WaitFor {
    fn update(&mut self, dt: Duration) {
        self.elapsed += dt
    }

    fn is_finished(&self) -> bool {
        self.elapsed > self.wait_duration
    }
}

impl Display for WaitFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Waiting for {:?} / {:?}",
            self.elapsed, self.wait_duration
        )
    }
}

pub struct WaitUntil {
    condition: Box<dyn Fn() -> bool + Send + Sync>,
    name: String
}

impl WaitUntil {
    pub fn new<F>(condition_name: &str, condition: F) -> Self
    where
        F: Fn() -> bool + 'static + Send + Sync,
    {
        Self {
            name: condition_name.into(),
            condition: Box::new(condition),
        }
    }
}

impl Action for WaitUntil {
    fn is_finished(&self) -> bool {
        (self.condition)()
    }
}

impl Display for WaitUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Waiting until {}...", self.name)
    }
}

pub struct UntilTimeout {
    action: Box<dyn Action>,
    timeout: Duration,
    elapsed: Duration,
}

impl UntilTimeout {
    pub fn new<A>(action: A, timeout: Duration) -> Self
    where
        A: Action + 'static,
    {
        Self {
            action: Box::new(action),
            timeout: timeout,
            elapsed: Duration::ZERO,
        }
    }
}

impl Action for UntilTimeout {
    fn start(&mut self) {
        self.action.start();
    }

    fn update(&mut self, dt: Duration) {
        self.action.update(dt);
        self.elapsed += dt;
    }
    fn is_finished(&self) -> bool {
        self.action.is_finished() || self.elapsed > self.timeout
    }

    fn stop(&mut self) {
        self.action.stop();
    }

}

impl Display for UntilTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}Until Timeout: {:?}/{:?}",
            self.action, self.elapsed, self.timeout
        )
    }
}
