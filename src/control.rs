pub mod actions;
pub mod landmark;
pub mod motion;
pub mod odometry;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::ROBOT;
use crate::control::actions::Action;
use crate::control::actions::navigate::Navigate;
use crate::control::landmark::Landmark;

use crate::math::{PidController, Pose, Twist};

pub fn spawn_main_controller_thread() {
    std::thread::spawn(|| {
        let mut controller = Controller::new();


        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(10) {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            }
            if controller.action_queue.is_empty() {
                controller
                    .enqueue(
                        Navigate::to_landmark(Landmark::QrZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SourceZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SideIntersection)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::TemporaryStorageZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FirstCornerTurn)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FinalProcessingZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SecondCornerTurn)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SourceZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SideIntersection)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::TemporaryStorageZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FirstCornerTurn)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FinalProcessingZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SecondCornerTurn)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SourceZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SideIntersection)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::TemporaryStorageZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FirstCornerTurn)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::FinalProcessingZone)
                    )
                    .enqueue(
                        Navigate::to_landmark(Landmark::SecondCornerTurn)
                    )
                ;
            }
            controller.update(dt);

            controller.state.publish();
            last_tick = now;
        }
    });
}

#[derive(Debug, Default)]
pub struct Controller {
    pub action_queue: VecDeque<Box<dyn Action>>,
    pub current_action: Option<Box<dyn Action>>,
    pub state: ControllerState
}

impl Controller {
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

    pub fn update(&mut self, dt: Duration) {
        if self.current_action.is_none() {
            self.current_action = self.action_queue.pop_front();
            return;
        } 
        
        if let Some(mut action) = self.current_action.take() {
            action.update(&mut self.state, dt);
            if action.is_finished() {
                action.stop(&mut self.state);
                self.current_action = None;
            } else {
                self.current_action = Some(action);
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ControllerState {
    pub target_pose: Pose,
    pub target_twist: Twist,

    pub linear_pid: PidController,
    pub angular_pid: PidController,

    pub dt: Duration
}

impl ControllerState {
    pub fn publish(&self) {
        ROBOT.controller_state.store(Arc::new(*self));
    }
}