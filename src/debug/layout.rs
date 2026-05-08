use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::debug::widgets::{
    compass::draw_compass, gyro::draw_gyro, maixcam::draw_maixcam, map::draw_map,
    motion::draw_motion, qr::draw_qr, stm32::draw_stm32, system::draw_system,
};

pub fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // system
            Constraint::Percentage(60), // devices
        ])
        .split(f.size());

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // top
            Constraint::Percentage(60), // bottom
        ])
        .split(chunks[1]);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),    // motion
            Constraint::Length(27), // compass
        ])
        .split(right_chunk[0]);

    let right_bottom_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // top
            Constraint::Percentage(80), // bottom
        ])
        .split(right_chunk[1]);

    let device_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1), // gyro
            Constraint::Fill(1), // qr
        ])
        .split(right_bottom_chunk[1]);

    let device_chunk_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1), // gyro
            Constraint::Fill(1), // qr
        ])
        .split(device_chunk[0]);

    let device_chunk_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1), // gyro
            Constraint::Fill(1), // qr
        ])
        .split(device_chunk[1]);

    draw_map(f, chunks[0]);
    draw_system(f, right_bottom_chunk[0]);
    draw_motion(f, top_chunks[0]);
    draw_compass(f, top_chunks[1]);
    draw_gyro(f, device_chunk_top[0]);
    draw_qr(f, device_chunk_top[1]);
    draw_maixcam(f, device_chunk_bottom[0]);
    draw_stm32(f, device_chunk_bottom[1]);
}
