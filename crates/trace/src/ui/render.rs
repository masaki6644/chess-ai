use ratatui::{
    Frame,
    layout::{
        Constraint,
        Direction,
        Layout,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
};

use crate::ui::app::AppState;

pub fn render(
    frame: &mut Frame,
    state: &AppState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // progress
    let progress = Paragraph::new(format!(
        "Progress: {} / {}",
        state.completed_files,
        state.total_files,
    ))
    .block(
        Block::default()
            .title("Progress")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        progress,
        chunks[0],
    );

    // workers
    let workers = state
        .workers
        .iter()
        .enumerate()
        .map(|(i, w)| {
            match &w.current_file {

                Some(path) => {
                    format!(
                        "Runner{}: {}",
                        i,
                        path,
                    )
                }

                None => {
                    format!(
                        "Runner{}: idle",
                        i,
                    )
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let workers_widget =
        Paragraph::new(workers)
        .block(
            Block::default()
                .title("Workers")
                .borders(Borders::ALL),
        );

    frame.render_widget(
        workers_widget,
        chunks[1],
    );

    // stats
    let stats = Paragraph::new(format!(
        "games: {}\nerrors: {}",
        state.games_seen,
        state.errors,
    ))
    .block(
        Block::default()
            .title("Stats")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        stats,
        chunks[2],
    );
}