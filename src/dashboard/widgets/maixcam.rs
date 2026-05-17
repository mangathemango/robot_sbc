use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::dashboard::helpers::{paragraph, bool_icon};

pub fn draw_maixcam(f: &mut Frame, area: Rect) {
    let maixcam = ROBOT.get_maixcam_state();

    let text = format!(
        "Connected: {}\nCircles {:?}\nFPS: {:.1}",
        bool_icon(maixcam.driver_is_connected),
        maixcam.circles,
        if maixcam.dt.as_secs_f32() > 0.0 { 1.0 / maixcam.dt.as_secs_f32() } else { 0.0 } as i32,
    );

    let block = Block::default()
        .title("MAIXCAM")
        .borders(Borders::ALL)
        .style(Style::default().fg(if maixcam.driver_is_connected {Color::Green} else {Color::Red}));

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}