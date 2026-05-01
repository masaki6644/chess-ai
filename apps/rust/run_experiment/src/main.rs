use std::fs::File;
use std::io::BufReader;

use experiment::runner::run;
use experiment::config::ExperimentConfig;

use pipeline::filter::NoFilter;
use pipeline::score::DummyScorer;
use pipeline::select::NoSelect;

use trace::collector::TraceCollector; // ← 追加

fn main() {
    let file = File::open("data/pgn/part_000.pgn").unwrap();
    let reader = BufReader::new(file);

    let filter = NoFilter;
    let scorer = DummyScorer;
    let selector = NoSelect;

    let config = ExperimentConfig {
        filter: &filter,
        scorer: &scorer,
        selector: &selector,
    };

    // 👇 追加
    let mut trace = TraceCollector::new();

    // 👇 引数追加
    run(reader, config, &mut trace);

    // 👇 最終サマリ
    trace.print_summary();
}