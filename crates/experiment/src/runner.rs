use std::io::BufRead;

use crossbeam::channel::Sender;

use pgn::parser::parse_pgn;

use pipeline::candidate::expand::expand;

use pipeline::types::{
    CandidateBatch,
    CandidatePosition,
};

use shakmaty::fen::Fen;
use shakmaty::Position;
use shakmaty::EnPassantMode;

use trace::event::TraceEvent;
use trace::meta::GameMeta;

use crate::config::ExperimentConfig;

pub fn run<R, F>(
    reader: R,

    config: ExperimentConfig<F>,

    sender: Sender<TraceEvent>,

    candidate_tx:
        Sender<CandidateBatch<F>>,

    file_id: u64,
    worker_id: usize,

    path: String,
)
where
    R: BufRead,

    F: Clone + Send + 'static,
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

    // =========================
    // streaming parse
    // =========================
    let mut game_index = 0usize;

    parse_pgn(reader, |game| {

        let i = game_index;

        game_index += 1;

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
                    .send(
                        TraceEvent::GameAccepted
                    )
                    .expect(
                        "trace send failed"
                    );
            }

            Err(reason) => {

                sender
                    .send(
                        TraceEvent::GameFiltered {
                            reason,
                            meta,
                        }
                    )
                    .expect(
                        "trace send failed"
                    );

                return;
            }
        }

        // =========================
        // expand
        // =========================
        let samples = expand(&game);

        let total_plies =
            samples
                .first()
                .map(|s| s.total_plies)
                .unwrap_or(0);

        sender
            .send(TraceEvent::Expanded {
                count: samples.len(),
                total_plies,
            })
            .expect("trace send failed");

        // =========================
        // feature
        // =========================
        let featured: Vec<_> =
            samples
                .into_iter()
                .map(|s| {

                    let f =
                        config
                            .feature_builder
                            .build(&s);

                    (s, f)
                })
                .collect();

        // =========================
        // score
        // =========================
        let scored: Vec<_> =
            featured
                .into_iter()
                .map(|(s, f)| {

                    config
                        .scorer
                        .score(s, f)
                })
                .collect();

        let scores =
            scored
                .iter()
                .map(|s| s.score)
                .collect();

        sender
            .send(TraceEvent::Scored {
                count: scored.len(),
                scores,
            })
            .expect("trace send failed");

        // =========================
        // select
        // =========================
        let selected =
            config
                .selector
                .select(scored);

        let selected_scores =
            selected
                .iter()
                .map(|s| s.score)
                .collect();

        sender
            .send(TraceEvent::Selected {
                count: selected.len(),
                scores: selected_scores,
            })
            .expect("trace send failed");

        // =========================
        // candidate batch
        // =========================
        let positions =
            selected
                .into_iter()
                .map(|s| {

                    let stm =
                        s.sample
                            .pos
                            .turn();

                    CandidatePosition {

                        // -----------------
                        // identity
                        // -----------------
                        game_id,

                        ply:
                            s.sample.ply as u16,

                        // -----------------
                        // position
                        // -----------------
                        fen:
                            Fen::from_position(
                                s.sample.pos,
                                EnPassantMode::Legal,
                            )
                                .to_string(),

                        stm,

                        // -----------------
                        // features
                        // -----------------
                        features:
                            s.features,

                        // -----------------
                        // candidate score
                        // -----------------
                        score:
                            s.score,
                    }
                })
                .collect();

        let batch =
            CandidateBatch {
                positions,
            };

        candidate_tx
            .send(batch)
            .expect(
                "failed to send candidate batch"
            );

        // NOTE:
        // selected dropped here
    });

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