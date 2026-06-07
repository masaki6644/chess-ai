use std::sync::Arc;

use crossbeam::channel;

use experiment::config::ExperimentConfig;

use pipeline::candidate::feature::{
    SimpleFeatureBuilder,
    SimpleFeatures,
};

use pipeline::candidate::filter::{
    StrongGameFilter,
    StrongGameFilterConfig,
};

use pipeline::candidate::score::QuickScorer;
use pipeline::candidate::select::SoftSelector;

use pipeline::labeling::mcts_factory::MctsLabelerFactory;
use pipeline::labeling::dummy_factory::DummyLabelerFactory;

use trace::analytics::summary::SummaryPrinter;
use trace::event::TraceEvent;

use engine::stockfish::config::StockfishConfig;

use crate::app::config::AppConfig;

use crate::channels::queues::PipelineQueues;

use crate::jobs::pgn::{
    enqueue_pgn_jobs,
    list_pgn_files,
};

use crate::runtime::analytics::spawn_analytics;
use crate::runtime::label_workers::spawn_label_workers;
use crate::runtime::parse_workers::spawn_parse_workers;
use crate::runtime::trace::spawn_trace_bus;
use crate::runtime::ui::spawn_ui;
use crate::runtime::writer::spawn_writer;

pub fn run_app() {

    // =========================
    // files
    // =========================
    let files =
        list_pgn_files("data/pgn");

    let total_files =
        files.len();

    // =========================
    // app config
    // =========================
    let app_config =
        AppConfig::default();

    // =========================
    // experiment config
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
            Arc::new(
                SoftSelector {
                    temperature: 0.03,
                    k: 16,
                }
            ),
    };

    // =========================
    // trace channels
    // =========================
    let (tx, rx) =
        channel::unbounded::<TraceEvent>();

    let (ui_tx, ui_rx) =
        channel::bounded::<TraceEvent>(
            1024,
        );

    let (ana_tx, ana_rx) =
        channel::unbounded::<TraceEvent>();

    // =========================
    // pipeline queues
    // =========================
    let queues =
        PipelineQueues::<SimpleFeatures>::new(
            app_config
                .candidate_queue_size,

            app_config
            .labeled_queue_size,
        );

    // =========================
    // runtime
    // =========================
    let bus_handle =
        spawn_trace_bus(
            rx,
            ui_tx,
            ana_tx,
        );

    let ui_handle =
        spawn_ui(
            ui_rx,
            total_files,
            app_config.parse_workers,
            app_config.label_workers
        );

    let analytics_handle =
        spawn_analytics(
            ana_rx,
        );

    let labeler_handles =
        spawn_label_workers(

            app_config.label_workers,

            queues
                .candidate_rx
                .clone(),

            queues
                .labeled_tx
                .clone(),

            tx.clone(),
            
            DummyLabelerFactory,
        
        );

    let writer_handle =
        spawn_writer(
            queues
                .labeled_rx
                .clone(),

            tx.clone()
        );

    // =========================
    // jobs
    // =========================
    let (
        job_tx,
        job_rx,
    ) = channel::unbounded();

    enqueue_pgn_jobs(
        &files,
        job_tx.clone(),
    );

    drop(job_tx);

    // =========================
    // parse workers
    // =========================
    let parse_handles =
        spawn_parse_workers(

            app_config
                .parse_workers,

            job_rx,

            config,

            tx.clone(),

            queues
                .candidate_tx
                .clone(),
        );

    // =========================
    // shutdown
    // =========================
    drop(tx);

    drop(
        queues
            .candidate_tx
    );

    drop(
        queues
            .labeled_tx
    );

    for h in parse_handles {
        h.join().unwrap();
    }

    for h in labeler_handles {
        h.join().unwrap();
    }

    writer_handle
        .join()
        .unwrap();

    bus_handle
        .join()
        .unwrap();

    ui_handle
        .join()
        .unwrap();

    let analytics =
        analytics_handle
            .join()
            .unwrap();

    SummaryPrinter::print(
        &analytics,
    );
}