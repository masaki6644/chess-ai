use std::io::BufRead;

use pgn::parser::parse_pgn;
use pipeline::expand::expand;

use trace::collector::TraceCollector;
use trace::event::TraceEvent;

use crate::config::ExperimentConfig;

pub fn run<R: BufRead>(
    reader: R,
    config: ExperimentConfig,
    trace: &mut TraceCollector,
) {
    let games = parse_pgn(reader);

    for game in games {

        // 👇 全ゲーム観測
        trace.record(TraceEvent::GameSeen);

        // ===== filter =====
        if !config.filter.accept(&game) {
            trace.record(TraceEvent::GameFiltered);
            continue;
        }

        trace.record(TraceEvent::GameAccepted);

        // ===== expand =====
        let positions = expand(&game);
        trace.record(TraceEvent::Expanded {
            positions: positions.len(),
        });

        // ===== score =====
        let scored: Vec<_> = positions
            .into_iter()
            .map(|p| config.scorer.score(p))
            .collect();

        trace.record(TraceEvent::Scored {
            positions: scored.len(),
        });

        // ===== select =====
        let selected = config.selector.select(scored);

        trace.record(TraceEvent::Selected {
            positions: selected.len(),
        });

        // 👇 デバッグ用（残してOK）
        println!("selected: {}", selected.len());
    }
}