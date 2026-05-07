use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use std::sync::Arc;

use crate::devices::gyro::GyroState;
use crate::devices::qr::QrState;
use crate::devices::stm32::Stm32State;

use crate::debug::widgets::{gyro::draw_gyro, stm32::draw_stm32, qr::draw_qr, system::draw_system};

pub fn ui(
    f: &mut Frame,
    gyro: &Arc<GyroState>,
    stm32: &Arc<Stm32State>,
    qr: &Arc<QrState>,
    history: &Vec<(f64, f64)>,
) {
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

    draw_gyro(f, right_chunks[0], gyro, history);
    draw_stm32(f, bottom_right_chunks[0], stm32);
    draw_qr(f, bottom_right_chunks[1], qr);
    draw_system(f, chunks[0]);
}