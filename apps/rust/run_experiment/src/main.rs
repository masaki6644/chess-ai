use std::fs::{self, File};
use std::io::BufReader;
use std::sync::Arc;
use std::thread;

use crossbeam::channel;

use experiment::config::ExperimentConfig;
use experiment::runner::run;

use pipeline::feature::{
    SimpleFeatureBuilder,
    SimpleFeatures,
};

use pipeline::filter::{
    StrongGameFilter,
    StrongGameFilterConfig,
};

use pipeline::score::QuickScorer;
use pipeline::select::NoSelect;

use trace::analytics::collector::TraceAnalytics;
use trace::analytics::summary::SummaryPrinter;

use trace::bus::TraceBus;

use trace::event::TraceEvent;

// =========================
// PGN一覧
// =========================
fn list_pgn_files(
    dir: &str,
) -> Vec<String> {

    let mut files = Vec::new();

    for entry in fs::read_dir(dir)
        .expect("failed to read dir")
    {
        let entry =
            entry.expect("invalid entry");

        let path = entry.path();

        if path
            .extension()
            .map(|e| e == "pgn")
            .unwrap_or(false)
        {
            files.push(
                path
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    files.sort();

    files
}

fn main() {

    // =========================
    // files
    // =========================
    let files =
        list_pgn_files("data/pgn");

    let total_files =
        files.len();

    println!(
        "found {} files",
        total_files,
    );

    // =========================
    // config
    // =========================
    let config:
        ExperimentConfig<SimpleFeatures> =
        ExperimentConfig {

        filter: Arc::new(
            StrongGameFilter {
                config:
                    StrongGameFilterConfig {
                    min_len: 15,
                    max_len: 120,

                    min_elo: 1300,
                    max_elo: 2400,
                },
            },
        ),

        feature_builder:
            Arc::new(SimpleFeatureBuilder),

        scorer:
            Arc::new(QuickScorer),

        selector:
            Arc::new(NoSelect),
    };

    // =========================
    // workers
    // =========================
    let base =
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

    let num_workers =
        base.saturating_sub(2).max(1);

    println!(
        "workers: {}",
        num_workers,
    );

    // =========================
    // channels
    // =========================
    let (tx, rx) =
        channel::unbounded::<TraceEvent>();

    // UIはdrop許容
    let (ui_tx, ui_rx) =
        channel::bounded::<TraceEvent>(
            1024,
        );

    // analyticsは完全保持
    let (ana_tx, ana_rx) =
        channel::unbounded::<TraceEvent>();

    // =========================
    // BUS THREAD
    // =========================
    let bus_handle =
        thread::spawn(move || {

        let bus = TraceBus::new(
            rx,
            ui_tx,
            ana_tx,
        );

        bus.run();
    });

    // =========================
    // UI THREAD
    // =========================
    let ui_handle =
        thread::spawn(move || {

        trace::ui::ui_loop::run_ui_loop(
            ui_rx,
            total_files,
            num_workers,
        );
    });

    // =========================
    // ANALYTICS THREAD
    // =========================
    let analytics_handle =
        thread::spawn(move || {

        let mut analytics =
            TraceAnalytics::new();

        for event in ana_rx {
            analytics.ingest(event);
        }

        SummaryPrinter::print(
            &analytics,
        );
    });

    // =========================
    // jobs
    // =========================
    let (job_tx, job_rx) =
        channel::unbounded::<(
            usize,
            String,
        )>();

    for (file_id, path)
        in files.into_iter().enumerate()
    {
        job_tx
            .send((file_id, path))
            .unwrap();
    }

    drop(job_tx);

    // =========================
    // worker threads
    // =========================
    let mut handles = vec![];

    for worker_id
        in 0..num_workers
    {
        let job_rx =
            job_rx.clone();

        let tx =
            tx.clone();

        let config =
            config.clone();

        handles.push(
            thread::spawn(move || {

            for (
                file_id,
                path,
            ) in job_rx
            {
                let file =
                    File::open(&path)
                        .expect(
                            "failed to open file",
                        );

                let reader =
                    BufReader::new(file);

                run(
                    reader,
                    config.clone(),
                    tx.clone(),

                    file_id as u64,
                    worker_id,
                    path.clone(),
                );
            }
        }));
    }

    // =========================
    // drop main sender
    // =========================
    drop(tx);

    // =========================
    // worker shutdown
    // =========================
    for h in handles {
        h.join().unwrap();
    }

    // =========================
    // bus shutdown
    // =========================
    bus_handle.join().unwrap();

    // =========================
    // ui shutdown
    // =========================
    ui_handle
        .join()
        .unwrap();

    // =========================
    // analytics shutdown
    // =========================
    analytics_handle
        .join()
        .unwrap();
}