// crates/experiment/src/label_runner.rs

use crossbeam::channel::{
    Receiver,
    Sender,
};

use pipeline::labeling::worker::Labeler;

use pipeline::types::{
    CandidateBatch,
    LabeledBatch,
};

use trace::event::{
    TraceEvent,
    WorkerKind,
    WorkerStatus,
};

pub fn run<F, L>(

    worker_id: usize,

    candidate_rx:
        Receiver<CandidateBatch<F>>,

    labeled_tx:
        Sender<LabeledBatch<F>>,

    trace_tx:
        Sender<TraceEvent>,

    labeler: L,
)
where
    F:
        Send
        + 'static,

    L:
        Labeler<F>,
{
    trace_tx
        .send(
            TraceEvent::WorkerStateUpdated {

                kind:
                    WorkerKind::Label,

                worker_id,

                status:
                    WorkerStatus::Idle,
            }
        )
        .ok();

    for batch in candidate_rx {

        // =========================
        // worker busy
        // =========================
        trace_tx
            .send(
                TraceEvent::WorkerStateUpdated {

                    kind:
                        WorkerKind::Label,

                    worker_id,

                    status:
                        WorkerStatus::Working {
                            task:
                                "labeling".into(),
                        },
                }
            )
            .ok();

        // =========================
        // labeling
        // =========================
        let labeled =
            labeler.label(batch);

        // =========================
        // send labeled
        // =========================
        labeled_tx
            .send(labeled)
            .unwrap();

        // =========================
        // labeled queue
        // =========================
        trace_tx
            .send(
                TraceEvent::LabeledQueue {

                    current:
                        labeled_tx.len(),

                    max:
                        labeled_tx
                            .capacity()
                            .unwrap_or(0),
                }
            )
            .ok();

        // =========================
        // worker idle
        // =========================
        trace_tx
            .send(
                TraceEvent::WorkerStateUpdated {

                    kind:
                        WorkerKind::Label,

                    worker_id,

                    status:
                        WorkerStatus::Idle,
                }
            )
            .ok();
    }
}