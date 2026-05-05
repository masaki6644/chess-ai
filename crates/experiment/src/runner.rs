use std::io::BufRead;

use pgn::parser::parse_pgn;
use pipeline::expand::expand;

use trace::collector::TraceCollector;
use trace::event::TraceEvent;

use crate::config::ExperimentConfig;

pub fn run<R, F>(
    reader: R,
    config: ExperimentConfig<F>,
    trace: &mut TraceCollector,
)
where
    R: BufRead,
    F: Clone,
{
    let games = parse_pgn(reader);

    for game in games {

        // ===== Game =====
        trace.record(TraceEvent::GameSeen);

        // ===== filter =====
        match config.filter.check(&game) {
            Ok(_) => {
                trace.record(TraceEvent::GameAccepted);
            }
            Err(reason) => {
                trace.record(TraceEvent::GameFiltered { reason,game });
                continue;
            }
        }

        // ===== expand =====
        let samples = expand(&game);
        trace.record(TraceEvent::Expanded {
            count: samples.len(),
        });

        // ===== feature ===== ★追加
        let featured: Vec<_> = samples
            .into_iter()
            .map(|s| {
                let f = config.feature_builder.build(&s);
                (s, f)
            })
            .collect();

        // ===== score ===== ★変更
        let scored: Vec<_> = featured
            .into_iter()
            .map(|(s, f)| config.scorer.score(s, f))
            .collect();

        trace.record(TraceEvent::Scored {
            count: scored.len(),
        });

        // ===== select =====
        let selected = config.selector.select(scored);

        trace.record(TraceEvent::Selected {
            count: selected.len(),
        });

        // debug
        //println!("selected: {}", selected.len());
    }
}