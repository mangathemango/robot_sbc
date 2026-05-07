use ratatui::{Frame, layout::Rect};

use std::sync::Arc;

use crate::devices::qr::QrState;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_qr(f: &mut Frame, area: Rect, qr: &Arc<QrState>) {
    let error_text = if qr.error_msg.is_empty() || qr.driver_is_connected {
        ""
    } else {
        &qr.error_msg
    };

    let text = format!(
        "Qr Code: {}\nConnected: {}\n{}",
        qr.code,
        bool_icon(qr.driver_is_connected),
        error_text
    );

    paragraph(f, area, "QR", text);
}