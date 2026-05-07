use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_maixcam(f: &mut Frame, area: Rect) {
    let maixcam = ROBOT.maixcam_state.load();

    let text = format!(
        "Connected: {}\nPosition: {:.1}, {:.1}\nColor: {:?}",
        bool_icon(maixcam.driver_is_connected),
        maixcam.circle_position.x,
        maixcam.circle_position.y,
        maixcam.circle_color,
    );

    paragraph(f, area, "MAIXCAM", text);
}