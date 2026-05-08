use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{format_radian, paragraph};

pub fn draw_odometry(f: &mut Frame, area: Rect) {
    let kinematic_state = ROBOT.odometry_state.load();
    let text = format!(
        "Current Twist: {}\nCurrent Pose: {}\nInitial Yaw: {}\nFPS: {:.1}",
        kinematic_state.current_twist,
        kinematic_state.current_pose,
        format_radian(kinematic_state.initial_rotation),
        if kinematic_state.dt.as_secs_f32() > 0.0 {
            1.0 / kinematic_state.dt.as_secs_f32()
        } else {
            0.0
        } as i32
    );

    paragraph(f, area, "ODOMETRY", text);
}
