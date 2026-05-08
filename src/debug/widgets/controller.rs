use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::debug::helpers::{paragraph, format_radian};

pub fn draw_controller(f: &mut Frame, area: Rect) {
    let controller_state = ROBOT.controller_state.load();

    let text = format!(
        "Target Pose: {}\nHeading: {}\nTarget Twist: {}\nDT: {:.2} ms\n\nLinear PID: {}\nAngular PID: {}",
        controller_state.target_pose,
        format_radian(controller_state.target_pose.rotation),
        controller_state.target_twist,
        controller_state.dt.as_secs_f32() * 1000.0,
        controller_state.linear_pid,
        controller_state.angular_pid,
    );

    paragraph(f, area, "CONTROLLER", text);
}
