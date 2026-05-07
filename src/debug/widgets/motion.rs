use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::paragraph;

pub fn draw_motion(f: &mut Frame, area: Rect) {
    let motion_state = ROBOT.motion_state.load();
    let text = format!(
        "Current Twist: {}\nTarget Twist: {}\nCurrent Pose: {}\nInitial Yaw: {}",
        motion_state.current_twist,
        motion_state.target_twist,
        motion_state.current_pose,
        motion_state.initial_rotation
    );

    paragraph(f, area, "MOTION", text);
}
