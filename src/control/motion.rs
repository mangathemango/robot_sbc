use crate::math::{Twist, Pose};
use crate::ROBOT;

#[derive(Debug,Clone,Copy,Default)]
struct MotionState {
    current_twist: Twist,
    target_twist: Twist,
    current_pose: Pose,

    initial_yaw: f32
}

impl MotionState {
    pub fn new(initial_yaw: f32) -> Self {
        Self {
            initial_yaw,
            ..Default::default()
        }
    } 

    pub fn update(&mut self) {
        let stm32_state = ROBOT.stm32_state.load();
        let gyro_state = ROBOT.gyro_state.load();

        self.current_pose.rotation = gyro_state.current_yaw;
    }
}



