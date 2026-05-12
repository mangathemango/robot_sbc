use std::time::Duration;

use crate::{
    ROBOT,
    control::{ControllerState, actions::Action, states::claw_servo::ClawPosition},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct RotateClaw {
    pub initial_angle: u8,
    pub target_angle: u8,
    pub elapsed_time: Duration,
}

impl RotateClaw {
    pub fn to(target_position: ClawPosition) -> Self {
        Self {
            target_angle: target_position.to_angle(),
            ..Default::default()
        }
    }
}

#[allow(unused_variables)]
impl Action for RotateClaw {
    fn start(&mut self, state: &mut ControllerState) {
        self.initial_angle = ROBOT.stm32_state.load().yaw_servo_state.current_angle;
        let stm32_controller = ROBOT.get_stm32_controller();
        stm32_controller.set_yaw_servo(self.target_angle);
    }

    fn update(&mut self, state: &mut ControllerState, dt: Duration) {
        self.elapsed_time += dt
    }

    fn is_finished(&self) -> bool {
        self.elapsed_time
            > Duration::from_millis(self.target_angle.abs_diff(self.initial_angle) as u64 * 20)
    }

    fn stop(&mut self, state: &mut ControllerState) {}
}
