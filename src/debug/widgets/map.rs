use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::debug::helpers::{format_radian, paragraph};

pub fn draw_map(f: &mut Frame, area: Rect) {
    let motion_state = ROBOT.motion_state.load();
    let map_text = build_pose_map(&motion_state.current_pose, 11);
    let text = format!(
        "{}\n\nPosition: {:.2}, {:.2}\nHeading: {}\nFPS: {:.1}",
        map_text,
        motion_state.current_pose.position.x,
        motion_state.current_pose.position.y,
        format_radian(motion_state.current_pose.rotation),
        if motion_state.dt.as_secs_f32() > 0.0 {
            1.0 / motion_state.dt.as_secs_f32()
        } else {
            0.0
        }
    );

    paragraph(f, area, "MAP", text);
}

fn build_pose_map(pose: &crate::math::Pose, size: usize) -> String {
    let height = size.max(5) | 1;
    let width = height * 3;
    let half_w = (width / 2) as isize;
    let half_h = (height / 2) as isize;

    let mut robot_x = (pose.position.x * 60.0).round() as isize;
    let mut robot_y = (pose.position.y * 20.0).round() as isize;
    robot_x = robot_x.clamp(-half_w, half_w);
    robot_y = robot_y.clamp(-half_h, half_h);

    let mut rows = Vec::with_capacity(height);
    for row in (0..height as isize).rev() {
        let mut line = String::with_capacity(width);
        for col in 0..width as isize {
            let x = col - half_w;
            let y = row - half_h;
            let ch = if x == robot_x && y == robot_y {
                'O'
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
