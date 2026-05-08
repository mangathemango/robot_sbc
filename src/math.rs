pub mod twist;
pub mod mecanum;
pub mod pid;
pub mod pose;

use std::f32::consts::PI;

pub use twist::Twist;
pub use mecanum::MecanumVelocities;
pub use pid::PidController;
pub use pose::Pose;

pub fn wrap_angle(rad: f32) -> f32 {
    let mut rad = rad;
    loop {

        if rad > PI {
            rad -= PI * 2.0;
        } else if rad < -PI {
            rad += PI * 2.0;
        } else {
            break;
        }
    }
    rad
}