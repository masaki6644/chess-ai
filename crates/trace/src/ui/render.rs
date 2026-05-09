use std::io::{
    stdout,
    Write,
};

use crate::ui::app::AppState;

pub fn render(state: &AppState) {

    // =========================
    // clear terminal
    // =========================
    print!("\x1B[2J\x1B[1;1H");

    // =========================
    // progress
    // =========================
    println!(
        "Progress: {} / {} files",
        state.completed_files,
        state.total_files,
    );

    println!();

    // =========================
    // workers
    // =========================
    for (i, worker)
        in state.workers.iter().enumerate()
    {
        match &worker.current_file {

            Some(path) => {
                println!(
                    "Runner{}: {}",
                    i,
                    path,
                );
            }

            None => {
                println!(
                    "Runner{}: idle",
                    i,
                );
            }
        }
    }

    println!();

    // =========================
    // counters
    // =========================
    println!(
        "games: {}",
        state.games_seen,
    );

    println!(
        "errors: {}",
        state.errors,
    );

    // =========================
    // force redraw
    // =========================
    stdout().flush().unwrap();
}