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

use crate::event::WorkerStatus;

use crate::ui::state::{
    AppState,
    WorkerState,
};

fn render_workers(
    prefix: &str,
    workers: &[WorkerState],
) -> String {

    workers
        .iter()
        .enumerate()
        .map(|(i, w)| {

            let text =
                match &w.status {

                    WorkerStatus::Idle => {
                        "idle".to_string()
                    }

                    WorkerStatus::Working {
                        task,
                    } => {
                        task.clone()
                    }
                };

            format!(
                "{}{:02}: {}",
                prefix,
                i,
                text,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render(
    frame: &mut Frame,
    state: &AppState,
) {

    let parser_height =
        state.parse_workers.len() as u16 + 2;

    let label_height =
        state.label_workers.len() as u16 + 2;

    let writer_height =
        state.writer_workers.len() as u16 + 3;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([

            Constraint::Length(4), // overall
            Constraint::Length(9), // queues
            Constraint::Length(9), // throughput

            Constraint::Length(
                parser_height,
            ),

            Constraint::Length(
                label_height,
            ),

            Constraint::Length(
                writer_height,
            ),

            Constraint::Length(4), // errors

        ])
        .split(frame.area());

    // =========================
    // overall
    // =========================
    let overall = Paragraph::new(format!(
        "Files : {} / {}\n\
         Games : {}",

        state.completed_files,
        state.total_files,

        state.games_seen,
    ))
    .block(
        Block::default()
            .title("Overall")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        overall,
        chunks[0],
    );

    // =========================
    // queues
    // =========================

    let candidate_avg =
        state
            .candidate_queue
            .avg()
            * 100.0;

    let candidate_peak =
        state
            .candidate_queue
            .peak()
            * 100.0;

    let labeled_avg =
        state
            .labeled_queue
            .avg()
            * 100.0;

    let labeled_peak =
        state
            .labeled_queue
            .peak()
            * 100.0;

    let queues = Paragraph::new(format!(
        "Candidate : {} / {}\n\
         Avg Util  : {:.1}%\n\
         Peak Util : {:.1}%\n\
         \n\
         Labeled   : {} / {}\n\
         Avg Util  : {:.1}%\n\
         Peak Util : {:.1}%",

        state.candidate_queue.current,
        state.candidate_queue.max,

        candidate_avg,
        candidate_peak,

        state.labeled_queue.current,
        state.labeled_queue.max,

        labeled_avg,
        labeled_peak,
    ))
    .block(
        Block::default()
            .title("Queues")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        queues,
        chunks[1],
    );

    // =========================
    // throughput
    // =========================

    let parse_rate =
        state
            .parse_throughput
            .per_sec();

    let label_rate =
        state
            .label_throughput
            .per_sec();

    let write_rate =
        state
            .write_throughput
            .per_sec();

    let throughput =
        Paragraph::new(format!(
            "Parse  : {} games\n\
             Rate   : {:.1} games/s\n\
             \n\
             Label  : {} positions\n\
             Rate   : {:.1} pos/s\n\
             \n\
             Write  : {} games\n\
             Rate   : {:.1} games/s",

            state.parse_throughput.total,
            parse_rate,

            state.label_throughput.total,
            label_rate,

            state.write_throughput.total,
            write_rate,
        ))
        .block(
            Block::default()
                .title("Throughput")
                .borders(Borders::ALL),
        );

    frame.render_widget(
        throughput,
        chunks[2],
    );

    // =========================
    // parser
    // =========================
    let parser = render_workers(
        "Parse",
        &state.parse_workers,
    );

    let parser_widget =
        Paragraph::new(parser)
        .block(
            Block::default()
                .title("Parser")
                .borders(Borders::ALL),
        );

    frame.render_widget(
        parser_widget,
        chunks[3],
    );

    // =========================
    // label
    // =========================
    let label = render_workers(
        "Label",
        &state.label_workers,
    );

    let label_widget =
        Paragraph::new(label)
        .block(
            Block::default()
                .title("Label")
                .borders(Borders::ALL),
        );

    frame.render_widget(
        label_widget,
        chunks[4],
    );

    // =========================
    // writer
    // =========================
    let writer = format!(
        "{}\n\
         Written : {} games",

        render_workers(
            "Writer",
            &state.writer_workers,
        ),

        state.written_games,
    );

    let writer_widget =
        Paragraph::new(writer)
        .block(
            Block::default()
                .title("Writer")
                .borders(Borders::ALL),
        );

    frame.render_widget(
        writer_widget,
        chunks[5],
    );

    // =========================
    // errors
    // =========================
    let errors = Paragraph::new(format!(
        "errors : {}",
        state.errors,
    ))
    .block(
        Block::default()
            .title("Errors")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        errors,
        chunks[6],
    );
}