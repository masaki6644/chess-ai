use std::fs::File;
use std::io::BufReader;

use experiment::runner::run;
use experiment::config::ExperimentConfig;

use pipeline::filter::{StrongGameFilter, StrongGameFilterConfig};
use pipeline::feature::{SimpleFeatureBuilder, SimpleFeatures};
use pipeline::score::QuickScorer;
use pipeline::select::NoSelect;

use trace::collector::TraceCollector; // ← 追加

fn main() {
    let file = File::open("data/pgn/part_000.pgn").unwrap();
    let reader = BufReader::new(file);

    let filter = StrongGameFilter {
        config: StrongGameFilterConfig {
            min_len: 15,
            max_len: 120,
            min_elo: 1300,
            max_elo: 2400,
        },
    };
    
    let feature_builder = SimpleFeatureBuilder;
    let scorer = QuickScorer;
    let selector = NoSelect;

    let config: ExperimentConfig<SimpleFeatures> = ExperimentConfig {
        filter: &filter,
        feature_builder: &feature_builder,
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