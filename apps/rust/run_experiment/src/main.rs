use std::fs::{self, File};
use std::io::BufReader;
use std::sync::Arc;
use std::thread;

use crossbeam::channel;

use experiment::runner::run;
use experiment::config::ExperimentConfig;

use pipeline::filter::{StrongGameFilter, StrongGameFilterConfig};
use pipeline::feature::{SimpleFeatureBuilder, SimpleFeatures};
use pipeline::score::QuickScorer;
use pipeline::select::NoSelect;

use trace::collector::TraceCollector;
use trace::event::TraceEvent;
use trace::summary::SummaryPrinter;

// =========================
// PGN一覧取得
// =========================
fn list_pgn_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).expect("failed to read dir") {
        let entry = entry.expect("invalid entry");
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "pgn" {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    files.sort();
    files
}

fn main() {
    // ===== ファイル一覧 =====
    let files = list_pgn_files("data/pgn");
    println!("found {} files", files.len());

    // ===== pipeline構成 =====
    let config: ExperimentConfig<SimpleFeatures> = ExperimentConfig {
        filter: Arc::new(StrongGameFilter {
            config: StrongGameFilterConfig {
                min_len: 15,
                max_len: 120,
                min_elo: 1300,
                max_elo: 2400,
            },
        }),
        feature_builder: Arc::new(SimpleFeatureBuilder),
        scorer: Arc::new(QuickScorer),
        selector: Arc::new(NoSelect),
    };

    // ===== worker数（2コア空け） =====
    let base = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let num_workers = base.saturating_sub(2).max(1);

    println!("workers: {}", num_workers);

    // ===== trace channel =====
    let (tx, rx) = channel::unbounded::<TraceEvent>();

    // ===== job channel =====
    let (job_tx, job_rx) = channel::unbounded::<(usize, String)>();

    // ===== job投入 =====
    for (file_id, path) in files.into_iter().enumerate() {
        job_tx.send((file_id, path)).unwrap();
    }
    drop(job_tx); // ★ 重要

    let mut handles = vec![];

    // ===== worker起動 =====
    for _ in 0..num_workers {
        let job_rx = job_rx.clone(); // ★ cloneできる（神）
        let tx = tx.clone();
        let config = config.clone();

        let handle = thread::spawn(move || {
            for (file_id, path) in job_rx {
                let file = File::open(&path).expect("failed to open file");
                let reader = BufReader::new(file);

                run(reader, config.clone(), tx.clone(), file_id as u64);
            }
        });

        handles.push(handle);
    }

    drop(tx); // ★ 重要

    // ===== collector =====
    let mut trace = TraceCollector::new();

    for event in rx {
        trace.record(event);
    }

    // ===== join =====
    for h in handles {
        h.join().unwrap();
    }

    // ===== summary =====
    SummaryPrinter::print(&trace);
}