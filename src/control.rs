pub mod actions;
pub mod claw_servo;
pub mod landmark;
pub mod motion;
pub mod odometry;
pub mod yaw_servo;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::{ROBOT, control};
use crate::control::actions::Action;
use crate::control::actions::r#move::Move;
use crate::control::actions::rotate_arm::RotateArm;
use crate::control::landmark::Landmark;

use crate::control::motion::MotionPolicyPreset;
use crate::control::yaw_servo::{ArmPosition, YawServoState};
use crate::math::Pose;

pub fn spawn_main_controller_thread() {
    std::thread::spawn(|| {
        let mut controller = Controller::new();

        let mut last_tick = Instant::now();
        loop {
            let now = Instant::now();
            let dt = now - last_tick;
            if dt < Duration::from_millis(20) {
                continue;
            }
            controller.state.dt = dt;
            if controller.action_queue.is_empty() {
                controller
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::QrZone))
                    .enqueue(Move::to(Landmark::SourceZone))
                    .enqueue(RotateArm::to(ArmPosition::Right))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::CentralRightCrossing))
                    .enqueue(Move::to(Landmark::TemporaryStorageZone))
                    .enqueue(RotateArm::to(ArmPosition::Left))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::UpperLeftTurn))
                    .enqueue(Move::to(Landmark::FinalProcessingZone))
                    .enqueue(RotateArm::to(ArmPosition::Right))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::UpperRightTurn))
                    .enqueue(Move::to(Landmark::SourceZone))
                    .enqueue(RotateArm::to(ArmPosition::Right))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::CentralRightCrossing))
                    .enqueue(Move::to(Landmark::TemporaryStorageZone))
                    .enqueue(RotateArm::to(ArmPosition::Left))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::UpperLeftTurn))
                    .enqueue(Move::to(Landmark::FinalProcessingZone))
                    .enqueue(RotateArm::to(ArmPosition::Right))
                    .enqueue(RotateArm::to(ArmPosition::Middle))
                    .enqueue(Move::to(Landmark::UpperRightTurn))
                    .enqueue(Move::to(Landmark::Start));
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
    pub state: ControllerState,
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

            if let Some(action) = &mut self.current_action {
                action.start(&mut self.state);
            }
            return;
        }

        if let Some(mut action) = self.current_action.take() {
            self.state.current_command = format!("{:?}", action);
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

#[derive(Debug, Default, Clone)]
pub struct ControllerState {
    pub current_command: String,
    pub target_pose: Pose,
    pub dt: Duration,
}

impl ControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&self) {
        ROBOT.controller_state.store(Arc::new(self.clone()));
    }
}
