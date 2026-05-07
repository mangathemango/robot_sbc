use std::{
    io, thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::ROBOT;
use crate::debug::layout::ui;

pub fn start() {
    thread::spawn(|| {
        if let Err(e) = run() {
            eprintln!("TUI error: {}", e);
        }
    })
    .join()
    .unwrap();
}

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
        // 🧠 Load latest gyro state for history
        let gyro = ROBOT.gyro_state.load();

        terminal.draw(|f| ui(f, &yaw_history))?;

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
