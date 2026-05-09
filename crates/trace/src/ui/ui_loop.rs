use std::time::Duration;

use crossbeam_channel::{
    Receiver,
    RecvTimeoutError,
};

use crate::event::TraceEvent;

use crate::ui::app::AppState;
use crate::ui::render::render;

pub fn run_ui_loop(
    rx: Receiver<TraceEvent>,
    total_files: usize,
    num_workers: usize,
) {

    let mut state =
        AppState::new(
            total_files,
            num_workers,
        );

    loop {

        match rx.recv_timeout(
            Duration::from_millis(16),
        ) {

            // =========================
            // received event
            // =========================
            Ok(event) => {

                state.ingest(event);

                // drain buffered events
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
        // redraw only if changed
        // =========================
        if state.dirty {

            render(&state);

            state.dirty = false;
        }
    }

    // =========================
    // final render
    // =========================
    render(&state);
}