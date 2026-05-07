use std::{
    f32::consts::PI, io, thread, time::{Duration, Instant}
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
};

use std::sync::Arc;

// 👇 IMPORT YOUR GLOBAL ROBOT
use crate::ROBOT;
use crate::devices::gyro::GyroState;
use crate::devices::qr::QrState;
use crate::devices::stm32::Stm32State;

// ====== PUBLIC ENTRY ======

pub fn start() {
    thread::spawn(|| {
        if let Err(e) = run() {
            eprintln!("TUI error: {}", e);
        }
    });
}

// ====== MAIN LOOP ======

fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut yaw_history: Vec<(f64, f64)> = Vec::new();
    let mut t: f64 = 0.0;

    loop {
        // 🧠 Load latest states
        let gyro = ROBOT.gyro_state.load();
        let stm32 = ROBOT.stm32_state.load();
        let qr = ROBOT.qr_state.load();

        terminal.draw(|f| ui(f, &gyro, &stm32, &qr, &yaw_history))?;

        // exit key (optional)
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let yaw = gyro.relative_yaw as f64;

            yaw_history.push((t, yaw));

            if yaw_history.len() > 100 {
                yaw_history.remove(0);
            }

            t += 1.0;
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// ====== UI ======

fn ui(
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
    draw_system(f, chunks[0]); // 👈 you’ll add this
}

fn draw_qr(f: &mut Frame, area: Rect, qr: &Arc<QrState>) {
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

    let block = Block::default().title("QR").borders(Borders::ALL);

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}

fn read_temperature() -> Option<f32> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .ok()?
            .trim()
            .parse::<f32>()
            .ok()
            .map(|v| v / 1000.0)
    }

    #[cfg(target_os = "windows")]
    {
        None // Windows is cursed for temps without extra APIs
    }

    #[cfg(target_os = "macos")]
    {
        None // same story unless using IOKit bindings
    }
}

// ====== PANELS ======

pub fn draw_system(f: &mut Frame, area: Rect) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    // RAM usage
    let total_mem = sys.total_memory() as f64;
    let used_mem = sys.used_memory() as f64;
    let mem_usage = (used_mem / total_mem) * 100.0;

    let text = format!(
        "SYSTEM\n\nCPU: {:.1}%\nRAM: {:.1}%\nTEMP: {}°C (ideal: 50-70°C)\n\nPROCS: {}",
        sys.global_cpu_usage(),
        mem_usage,
        read_temperature().unwrap_or(0.0),
        sys.processes().len()
    );

    let block = Block::default()
        .title("SYSTEM")
        .borders(Borders::ALL)
        .border_style(if sys.global_cpu_usage() > 80.0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        });

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);

    f.render_widget(p, area);
}

fn draw_gyro(f: &mut Frame, area: Rect, g: &GyroState, history: &Vec<(f64, f64)>) {
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
        .marker(symbols::Marker::Braille)
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

fn draw_compass(f: &mut Frame, area: Rect, yaw_deg: f32, size_x: usize, size_y: usize) {
    let mut grid = vec![vec![' '; size_x]; size_y];

    let cx = (size_x / 2) as f32;
    let cy = (size_y / 2) as f32;

    let rx = cx; // horizontal radius
    let ry = cy; // vertical radius

    // 🟣 draw circle (normalized)
    for y in 0..size_y {
        for x in 0..size_x {
            let dx = (x as f32 - cx) / rx;
            let dy = (y as f32 - cy) / ry;

            let dist = (dx * dx + dy * dy).sqrt();

            if (dist - 1.0).abs() < 0.08 {
                grid[y][x] = '•';
            }
        }
    }

    // 🟢 direction line
    let angle = yaw_deg;

    let steps = size_x.max(size_y);

    for i in 0..steps {
        let t = i as f32 / steps as f32;

        let x = cx - t * rx * angle.sin();
        let y = cy - t * ry * angle.cos();

        let xi = x.round() as usize;
        let yi = y.round() as usize;

        if xi < size_x && yi < size_y {
            grid[yi][xi] = '│';
        }
    }

    // 🔴 center
    grid[cy as usize][cx as usize] = 'O';

    // 🧭 cardinal directions
    if cy >= 1.0 {
        grid[0][cx as usize] = 'N';
        grid[size_y - 1][cx as usize] = 'S';
    }
    if cx >= 1.0 {
        grid[cy as usize][0] = 'W';
        grid[cy as usize][size_x - 1] = 'E';
    }

    let lines: Vec<Line> = grid
        .into_iter()
        .map(|row| {
            let spans: Vec<Span> = row
                .into_iter()
                .map(|ch| match ch {
                    'N' => Span::styled("N", Style::default().fg(Color::Red)),
                    'S' => Span::styled("S", Style::default().fg(Color::Red)),
                    'E' => Span::styled("E", Style::default().fg(Color::Blue)),
                    'W' => Span::styled("W", Style::default().fg(Color::Blue)),
                    'O' => Span::styled("O", Style::default().fg(Color::Yellow)),
                    '│' => Span::styled("│", Style::default().fg(Color::Green)),
                    '•' => Span::styled("•", Style::default().fg(Color::DarkGray)),
                    _ => Span::raw(ch.to_string()),
                })
                .collect();

            Line::from(spans)
        })
        .collect();
    let p = Paragraph::new(lines).block(Block::default().title("COMPASS").borders(Borders::ALL));

    f.render_widget(p, area);
}

fn draw_stm32(f: &mut Frame, area: Rect, s: &Stm32State) {
    let motion_state = ROBOT.motion_state.load();
    let text = format!(
        "Running: {}\nWheels: {:?}\nConnected: {}\n{:#?} ",
        bool_icon(s.start_flag),
        s.actual_wheel_velocities,
        bool_icon(s.driver_is_connected),
        motion_state.current_pose
    );

    paragraph(f, area, "STM32", text);
}

// ====== HELPERS ======

fn paragraph(f: &mut Frame, area: Rect, title: &str, text: String) {
    let block = Block::default().title(title).borders(Borders::ALL);
    let p = Paragraph::new(text).block(block);
    f.render_widget(p, area);
}

fn bool_icon(b: bool) -> &'static str {
    if b { "✅" } else { "❌" }
}
