use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::dashboard::helpers::{format_radian, paragraph};
use crate::math::Pose;

pub fn draw_map(f: &mut Frame, area: Rect) {
    let odometry_state = ROBOT.odometry_state();
    let map_text = build_pose_map(&odometry_state.pose, 21);
    let text = format!(
        "{}",
        map_text
    );

    paragraph(f, area, "MAP", text);
}

fn build_pose_map(pose: &Pose, size: usize) -> String {
    let height = size.max(5) | 1;
    let width = height * 2;
    let half_w = (width / 2) as isize;
    let half_h = (height / 2) as isize;

    let robot_x = (pose.position.x * 60.0).round() as isize;
    let robot_y = (pose.position.y * 30.0).round() as isize;

    let mut rows = Vec::with_capacity(height);
    for row in (0..height as isize).rev() {
        let mut line = String::with_capacity(width);
        for col in 0..width as isize {
            let x = col - half_w - 17;
            let y = row - half_h + 9;
            let ch = if x == robot_x && y == robot_y {
                'X'
            } else if x == 0 && y == 0 {
                '+'
            } else {
                '_'
            };
            line.push(ch);
        }
        rows.push(line);
    }

    rows.join("\n")
}
