use std::fmt::Display;

use crate::{ROBOT, control::actions::Action};

#[derive(Debug, Clone, Default)]
pub struct LiftArm {
    target_position: u16
}

impl LiftArm {
    pub fn to_position(target_position: u16) -> Self {
        Self {
            target_position
        }
    }

    pub fn to(position: LiftPosition) -> Self {
        Self {
            target_position: position.to_position()
        }
    }

    pub fn up() -> Self {Self::to(LiftPosition::Up)}
    pub fn storage() -> Self {Self::to(LiftPosition::Storage)}
    pub fn ground() -> Self {Self::to(LiftPosition::Ground)}
    pub fn stack() -> Self {Self::to(LiftPosition::Stack)}
}

impl Action for LiftArm {
    fn start(&mut self) {
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_vertical_arm_position(self.target_position);
    }

    fn update(&mut self, dt: std::time::Duration) {
        
    }

    fn stop(&mut self) {
        
    }
    fn current_action(&self) -> &dyn Action {
        self
    }

    fn is_finished(&self) -> bool {
        todo!("Implement get arm motor current position")
    }

    fn name(&self) -> String {
        "Lift Arm".into()
    }
}

impl Display for LiftArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lift Arm to {}", self.target_position)
    }
}

enum LiftPosition {
    Up,
    Storage,
    Ground, 
    Stack
}

impl LiftPosition {
    pub fn to_position(&self) -> u16 {
        match self {
            LiftPosition::Up => 10000,
            LiftPosition::Storage => 6000,
            LiftPosition::Ground => 1000,
            LiftPosition::Stack => 2000,
        }
    }
}

