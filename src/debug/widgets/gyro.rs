use std::f32::consts::PI;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Frame,
};

use crate::devices::gyro::GyroState;

use crate::debug::helpers::bool_icon;
use crate::debug::widgets::compass::draw_compass;

pub fn draw_gyro(f: &mut Frame, area: Rect, g: &GyroState, history: &Vec<(f64, f64)>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(35),
            Constraint::Length(27),
            Constraint::Fill(1),
        ])
        .split(area);

    draw_gyro_text(f, chunks[0], g);
    draw_compass(f, chunks[1], g.relative_yaw, 25, 11);
    draw_yaw_graph(f, chunks[2], history);
}

fn draw_gyro_text(f: &mut Frame, area: Rect, g: &GyroState) {
    let color = if !g.driver_is_connected {
        Color::Red
    } else if g.relative_yaw.abs() > 45.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    let text = format!(
        "Relative yaw: {:.2}π rad ({:.2}°)\nRaw yaw: {:.2}π rad ({:.2}°)\nInitial yaw: {:.2}π rad ({:.2}°)\nGY: {:.2}\nGZ: {:.2}\nConnected: {}\n{:?}",
        g.relative_yaw / PI, g.relative_yaw.to_degrees(),
        g.current_yaw  / PI, g.current_yaw .to_degrees(),
        g.initial_yaw  / PI, g.initial_yaw .to_degrees(),
        g.gy,
        g.gz,
        bool_icon(g.driver_is_connected),
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

fn draw_yaw_graph(f: &mut Frame, area: Rect, history: &Vec<(f64, f64)>) {
    let dataset = Dataset::default()
        .name("Yaw")
        .marker(ratatui::symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(history);

    let x_bounds = if history.is_empty() {
        [0.0, 100.0]
    } else {
        let min_x = history.first().unwrap().0;
        let max_x = history.last().unwrap().0;
        [min_x, max_x]
    };

    let y_bounds = [-PI as f64, PI as f64];

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("Yaw Graph").borders(Borders::ALL))
        .x_axis(Axis::default().title("t").bounds(x_bounds))
        .y_axis(Axis::default().title("deg").bounds(y_bounds));

    f.render_widget(chart, area);
}