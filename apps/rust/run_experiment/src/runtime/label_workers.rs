// apps/rust/run_experiment/src/runtime/label_workers.rs

use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{
    Receiver,
    Sender,
};

use experiment::label_runner;

use pipeline::labeling::worker::Labeler;

use pipeline::types::{
    CandidateBatch,
    LabeledBatch,
};

use trace::event::TraceEvent;

pub fn spawn_label_workers<F, L>(

    num_workers: usize,

    candidate_rx:
        Receiver<CandidateBatch<F>>,

    labeled_tx:
        Sender<LabeledBatch<F>>,

    trace_tx:
        Sender<TraceEvent>,

    labeler: L,
)
-> Vec<JoinHandle<()>>
where
    F:
        Send
        + 'static,

    L:
        Labeler<F>
        + Clone
        + Send
        + 'static,
{
    let mut handles =
        Vec::new();

    for worker_id in 0..num_workers {

        let candidate_rx =
            candidate_rx.clone();

        let labeled_tx =
            labeled_tx.clone();

        let trace_tx =
            trace_tx.clone();

        let labeler =
            labeler.clone();

        let handle =
            thread::spawn(move || {

                label_runner::run(

                    worker_id,

                    candidate_rx,

                    labeled_tx,

                    trace_tx,

                    labeler,
                );
            });

        handles.push(handle);
    }

    handles
}