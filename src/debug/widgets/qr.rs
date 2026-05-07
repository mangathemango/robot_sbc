use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_qr(f: &mut Frame, area: Rect) {
    let qr = ROBOT.qr_state.load();
    let error_text = if qr.error_msg.is_empty() || qr.driver_is_connected {
        ""
    } else {
        &qr.error_msg
    };

    let text = format!(
        "Qr Code: {}\nConnected: {}\nFPS: {:.1}\n{}",
        qr.code,
        bool_icon(qr.driver_is_connected),
        if qr.dt.as_secs_f32() > 0.0 { 1.0 / qr.dt.as_secs_f32() } else { 0.0 } as i32,
        error_text
    );

    paragraph(f, area, "QR", text);
}