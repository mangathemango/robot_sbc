use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::debug::widgets::{
    compass::draw_compass, gyro::draw_gyro, maixcam::draw_maixcam, motion::draw_motion,
    qr::draw_qr, stm32::draw_stm32, system::draw_system,
};

pub fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // system
            Constraint::Percentage(80), // devices
        ])
        .split(f.size());

    let devices_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // motion/compass
            Constraint::Percentage(50), // device widgets
        ])
        .split(chunks[1]);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),    // motion
            Constraint::Length(27), // compass
        ])
        .split(devices_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1), // gyro
            Constraint::Fill(1), // qr
            Constraint::Fill(1), // maixcam
            Constraint::Fill(1), // stm
        ])
        .split(devices_chunks[1]);

    draw_system(f, chunks[0]);
    draw_motion(f, top_chunks[0]);
    draw_compass(f, top_chunks[1]);
    draw_gyro(f, bottom_chunks[0]);
    draw_qr(f, bottom_chunks[1]);
    draw_maixcam(f, bottom_chunks[2]);
    draw_stm32(f, bottom_chunks[3]);
}
