use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::dashboard::helpers::paragraph;

pub fn draw_controller(f: &mut Frame, area: Rect) {
    let controller_state = ROBOT.get_controller_state();

    let text = format!(
        "Current Command: {}\nDT: {:.2} ms\n",
        controller_state.current_command_debug_string,
        controller_state.dt.as_secs_f32() * 1000.0,
    );

    paragraph(f, area, "CONTROLLER", text);
}
