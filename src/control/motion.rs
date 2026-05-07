use crate::math::{MecanumVelocities, Pose, Twist, mecanum};
use crate::ROBOT;

#[derive(Debug,Clone,Copy,Default)]
pub struct MotionState {
    pub current_twist: Twist,
    pub target_twist: Twist,
    pub current_pose: Pose,
    pub initial_yaw: f32
}

impl MotionState {
    pub fn new() -> Self {
        let gyro_state = ROBOT.gyro_state.load();
        let initial_yaw = if gyro_state.driver_is_connected {
            gyro_state.current_yaw
        } else {
            f32::NAN
        };
        
        Self {
            initial_yaw,
            ..Default::default()
        }
    } 

    pub fn update(&mut self) {
        let stm32_state = ROBOT.stm32_state.load();
        let gyro_state = ROBOT.gyro_state.load();

        if self.initial_yaw.is_nan() {
            self.initial_yaw = gyro_state.current_yaw;
        }
        
        let [vfl, vfr, vrl, vrr] = stm32_state.actual_wheel_velocities
            .map(|v| v as f32 / 10000.0);

        self.current_pose.rotation = gyro_state.current_yaw - self.initial_yaw;
        self.current_twist = Twist::from_mecanum_velocities(MecanumVelocities::new(vfl, vfr, vrl, vrr));
    }
}



