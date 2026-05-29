use std::thread::{
    self,
    JoinHandle,
};

use crossbeam::channel::{
    Receiver,
    Sender,
};

use experiment::label_runner;

use pipeline::labeling::factory::LabelerFactory;

use pipeline::types::{
    CandidateBatch,
    LabeledBatch,
};

use trace::event::TraceEvent;

pub fn spawn_label_workers<F, LF>(

    workers: usize,

    candidate_rx:
        Receiver<CandidateBatch<F>>,

    labeled_tx:
        Sender<LabeledBatch<F>>,

    trace_tx:
        Sender<TraceEvent>,

    factory: LF,
)
-> Vec<JoinHandle<()>>
where
    F:
        Send
        + 'static,

    LF:
        LabelerFactory<F>,
{
    let mut handles =
        Vec::new();

    for worker_id in 0..workers {

        let candidate_rx =
            candidate_rx.clone();

        let labeled_tx =
            labeled_tx.clone();

        let trace_tx =
            trace_tx.clone();

        let labeler =
            factory.create(worker_id);

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