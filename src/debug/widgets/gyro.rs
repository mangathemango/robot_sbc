use std::f32::consts::PI;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{ROBOT, debug::helpers::format_radian};
use crate::devices::gyro::GyroState;

use crate::debug::helpers::bool_icon;

pub fn draw_gyro(f: &mut Frame, area: Rect) {
    let g = ROBOT.gyro_state.load();
    draw_gyro_text(f, area, &*g);
}

fn draw_gyro_text(f: &mut Frame, area: Rect, g: &GyroState) {
    let color = if !g.driver_is_connected {
        Color::Red
    } else {
        Color::Green
    };

    let text = format!(
        "Raw yaw: {}\nGY: {:.2}\nGZ: {:.2}\nConnected: {}\nFPS: {:.1}\n{}",
        format_radian(g.yaw),
        g.gy,
        g.gz,
        bool_icon(g.driver_is_connected),
        if g.dt.as_secs_f32() > 0.0 {
            1.0 / g.dt.as_secs_f32()
        } else {
            0.0
        } as i32,
        match &g.error_msg {
            Some(msg) => msg,
            None => "",
        }
    );

    let block = Block::default()
        .title("GYRO")
        .borders(Borders::ALL)
        .style(Style::default().fg(color));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}
