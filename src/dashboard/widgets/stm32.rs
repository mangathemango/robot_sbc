use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::dashboard::helpers::{bool_icon, paragraph};

pub fn draw_stm32(f: &mut Frame, area: Rect) {
    let s = ROBOT.get_stm32_state();

    let text = format!("{}", s);

    paragraph(f, area, "STM32", text);
}
