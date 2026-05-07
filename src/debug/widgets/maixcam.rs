use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_maixcam(f: &mut Frame, area: Rect) {
    let maixcam = ROBOT.maixcam_state.load();

    let text = format!(
        "Connected: {}\nPosition: {:.1}, {:.1}\nColor: {:?}\nFPS: {:.1}",
        bool_icon(maixcam.driver_is_connected),
        maixcam.circle_position.x,
        maixcam.circle_position.y,
        maixcam.circle_color,
        if maixcam.dt.as_secs_f32() > 0.0 { 1.0 / maixcam.dt.as_secs_f32() } else { 0.0 } as i32,
    );

    paragraph(f, area, "MAIXCAM", text);
}