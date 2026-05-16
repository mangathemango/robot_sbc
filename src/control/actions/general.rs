use crate::control::actions::Action;
use std::{fmt::Display, time::Duration};

pub struct OneShot {
    f: Box<dyn FnMut()>,
}

impl OneShot {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static,
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

    fn current_action(&self) -> &dyn Action {
        self
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

    fn current_action(&self) -> &dyn Action {
        self
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
    condition: Box<dyn Fn() -> bool>,
}

impl WaitUntil {
    pub fn new<F>(condition: F) -> Self
    where
        F: Fn() -> bool + 'static,
    {
        Self {
            condition: Box::new(condition),
        }
    }
}

impl Action for WaitUntil {
    fn current_action(&self) -> &dyn Action {
        self
    }

    fn is_finished(&self) -> bool {
        (self.condition)()
    }
}

impl Display for WaitUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Waiting until custom condition...")
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
    fn current_action(&self) -> &dyn Action {
        self
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
