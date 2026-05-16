use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_qr(f: &mut Frame, area: Rect) {
    let qr = ROBOT.get_qr_state();
    let error_text = if qr.error_msg.is_empty() || qr.driver_is_connected {
        ""
    } else {
        &qr.error_msg
    };

    let text = format!(
        "Qr Code: {}\nConnected: {}\n{}",
        if qr.code.len() == 0 {"Waiting..."} else {&qr.code},
        bool_icon(qr.driver_is_connected),
        error_text
    );

    paragraph(f, area, "QR", text);
}