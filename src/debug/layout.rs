use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::debug::widgets::{gyro::draw_gyro, qr::draw_qr, stm32::draw_stm32, system::draw_system};

pub fn ui(f: &mut Frame, history: &Vec<(f64, f64)>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // system
            Constraint::Percentage(80), // robot
        ])
        .split(f.size());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // gyro
            Constraint::Percentage(50), // stm32
        ])
        .split(chunks[1]);

    let bottom_right_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(21)])
        .split(right_chunks[1]);

    draw_gyro(f, right_chunks[0], history);
    draw_stm32(f, bottom_right_chunks[0]);
    draw_qr(f, bottom_right_chunks[1]);
    draw_system(f, chunks[0]);
}
