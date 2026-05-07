use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::paragraph;

pub fn draw_motion(f: &mut Frame, area: Rect) {
    let motion_state = ROBOT.motion_state.load();
    let text = format!(
        "Current Twist: {}\nTarget Twist: {}\nCurrent Pose: {}\nInitial Yaw: {}\nFPS: {:.1}",
        motion_state.current_twist,
        motion_state.target_twist,
        motion_state.current_pose,
        motion_state.initial_rotation,
        if motion_state.dt.as_secs_f32() > 0.0 { 1.0 / motion_state.dt.as_secs_f32() } else { 0.0 } as i32
    );

    paragraph(f, area, "MOTION", text);
}
