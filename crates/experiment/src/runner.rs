use std::io::BufRead;

use crossbeam::channel::Sender;

use pgn::parser::parse_pgn;

use pipeline::expand::expand;

use trace::event::TraceEvent;
use trace::meta::GameMeta;

use crate::config::ExperimentConfig;

pub fn run<R, F>(
    reader: R,
    config: ExperimentConfig<F>,
    sender: Sender<TraceEvent>,

    file_id: u64,
    worker_id: usize,
    path: String,
)
where
    R: BufRead,
    F: Clone,
{
    // =========================
    // file started
    // =========================
    sender
        .send(TraceEvent::FileStarted {
            worker_id,
            file_id,
            path: path.clone(),
        })
        .expect("trace send failed");

    let games = parse_pgn(reader);

    for (i, game) in games.into_iter().enumerate() {

        // =========================
        // unique game id
        // =========================
        let game_id =
            (file_id << 32) | (i as u64);

        let meta = GameMeta {
            game_id,
            moves: game.moves.len(),
            white_elo: game.white_elo,
            black_elo: game.black_elo,
        };

        // =========================
        // game seen
        // =========================
        sender
            .send(TraceEvent::GameSeen)
            .expect("trace send failed");

        // =========================
        // filter
        // =========================
        match config.filter.check(&game) {

            Ok(_) => {
                sender
                    .send(TraceEvent::GameAccepted)
                    .expect("trace send failed");
            }

            Err(reason) => {
                sender
                    .send(
                        TraceEvent::GameFiltered {
                            reason,
                            meta,
                        }
                    )
                    .expect("trace send failed");

                continue;
            }
        }

        // =========================
        // expand
        // =========================
        let samples = expand(&game);

        sender
            .send(TraceEvent::Expanded {
                count: samples.len(),
            })
            .expect("trace send failed");

        // =========================
        // feature
        // =========================
        let featured: Vec<_> = samples
            .into_iter()
            .map(|s| {
                let f =
                    config.feature_builder.build(&s);

                (s, f)
            })
            .collect();

        // =========================
        // score
        // =========================
        let scored: Vec<_> = featured
            .into_iter()
            .map(|(s, f)| {
                config.scorer.score(s, f)
            })
            .collect();

        sender
            .send(TraceEvent::Scored {
                count: scored.len(),
            })
            .expect("trace send failed");

        // =========================
        // select
        // =========================
        let selected =
            config.selector.select(scored);

        sender
            .send(TraceEvent::Selected {
                count: selected.len(),
            })
            .expect("trace send failed");
    }

    // =========================
    // file finished
    // =========================
    sender
        .send(TraceEvent::FileFinished {
            worker_id,
            file_id,
        })
        .expect("trace send failed");
}