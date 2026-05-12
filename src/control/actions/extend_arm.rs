use crate::{ROBOT, control::actions::{Action, lift_arm::LiftArm}};

#[derive(Debug, Clone, Default)]
pub struct ExtendArm {
    target_position: u16,
}

impl ExtendArm {
    pub fn to_position(position: u16) -> Self {
        ExtendArm { target_position: position }
    }
}

impl Action for ExtendArm {
    fn start(&mut self) {
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_horizontal_arm_position(self.target_position);
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
}

enum ExtendPosition {
    
}