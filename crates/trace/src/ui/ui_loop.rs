use std::time::Duration;

use crossbeam_channel::{
    Receiver,
    RecvTimeoutError,
};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::event::TraceEvent;

use crate::ui::app::AppState;
use crate::ui::render::render;

pub fn run_ui_loop(
    rx: Receiver<TraceEvent>,
    total_files: usize,
    num_workers: usize,
) {

    // =========================
    // stdout
    // =========================
    let mut stdout =
        std::io::stdout();

    // =========================
    // terminal init
    // =========================
    enable_raw_mode().unwrap();

    execute!(
        stdout,
        EnterAlternateScreen
    )
    .unwrap();

    let backend =
        CrosstermBackend::new(stdout);

    let mut terminal =
        Terminal::new(backend)
            .unwrap();

    // =========================
    // state
    // =========================
    let mut state =
        AppState::new(
            total_files,
            num_workers,
        );

    // =========================
    // loop
    // =========================
    loop {

        match rx.recv_timeout(
            Duration::from_millis(16),
        ) {

            Ok(event) => {

                state.ingest(event);

                while let Ok(event) =
                    rx.try_recv()
                {
                    state.ingest(event);
                }
            }

            Err(
                RecvTimeoutError::Timeout
            ) => {}

            Err(
                RecvTimeoutError::Disconnected
            ) => {
                break;
            }
        }

        terminal
            .draw(|frame| {
                render(frame, &state);
            })
            .unwrap();
    }

    // =========================
    // restore terminal
    // =========================
    disable_raw_mode().unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )
    .unwrap();

    terminal
        .show_cursor()
        .unwrap();
}