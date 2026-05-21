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

    num_parse_workers: usize,

    num_label_workers: usize,
) {

    // =========================
    // stdout
    // =========================
    let mut stdout =
        std::io::stdout();

    // =========================
    // terminal init
    // =========================
    enable_raw_mode()
        .unwrap();

    execute!(
        stdout,
        EnterAlternateScreen,
    )
    .unwrap();

    let backend =
        CrosstermBackend::new(
            stdout,
        );

    let mut terminal =
        Terminal::new(backend)
            .unwrap();

    // =========================
    // state
    // =========================
    let mut state =
        AppState::new(

            total_files,

            num_parse_workers,

            num_label_workers,
        );

    // =========================
    // initial draw
    // =========================
    terminal
        .draw(|frame| {
            render(
                frame,
                &state,
            );
        })
        .unwrap();

    state.dirty = false;

    // =========================
    // loop
    // =========================
    loop {

        match rx.recv_timeout(
            Duration::from_millis(16),
        ) {

            // =========================
            // received event
            // =========================
            Ok(event) => {

                state.ingest(event);

                // =========================
                // drain burst
                // =========================
                while let Ok(event) =
                    rx.try_recv()
                {
                    state.ingest(event);
                }
            }

            // =========================
            // periodic wakeup
            // =========================
            Err(
                RecvTimeoutError::Timeout
            ) => {}

            // =========================
            // shutdown
            // =========================
            Err(
                RecvTimeoutError::Disconnected
            ) => {
                break;
            }
        }

        // =========================
        // redraw only if dirty
        // =========================
        if state.dirty {

            terminal
                .draw(|frame| {

                    render(
                        frame,
                        &state,
                    );

                })
                .unwrap();

            state.dirty = false;
        }
    }

    // =========================
    // restore terminal
    // =========================
    disable_raw_mode()
        .unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )
    .unwrap();

    terminal
        .show_cursor()
        .unwrap();
}