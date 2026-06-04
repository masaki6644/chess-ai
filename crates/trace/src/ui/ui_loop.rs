use std::io::{
    BufRead,
    BufReader,
};

use std::net::TcpStream;

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

    stream: TcpStream,

    _total_files: usize,

    _num_parse_workers: usize,

    _num_label_workers: usize,
) {

    let mut stdout =
        std::io::stdout();

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

    // 仮state
    let mut state =
        AppState::new(
            0,
            0,
            0,
        );

    terminal
        .draw(|frame| {

            render(
                frame,
                &state,
            );

        })
        .unwrap();

    state.dirty = false;

    let reader =
        BufReader::new(stream);

    for line in reader.lines() {

        let line =
            match line {

                Ok(line) => line,

                Err(_) => break,
            };

        let event: TraceEvent =
            match serde_json::from_str(
                &line,
            ) {

                Ok(event) => event,

                Err(_) => continue,
            };

        match event {

            TraceEvent::Init {

                total_files,

                num_parse_workers,

                num_label_workers,
            } => {

                state =
                    AppState::new(

                        total_files,

                        num_parse_workers,

                        num_label_workers,
                    );

                state.dirty = true;
            }

            _ => {

                state.ingest(event);
            }
        }

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