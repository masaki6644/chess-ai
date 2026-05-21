// crates/experiment/src/writer_runner.rs

use crossbeam::channel::{
    Receiver,
    Sender,
};

use pipeline::types::LabeledBatch;

use trace::event::{
    TraceEvent,
    WorkerKind,
    WorkerStatus,
};

pub fn run<F>(

    labeled_rx:
        Receiver<LabeledBatch<F>>,

    trace_tx:
        Sender<TraceEvent>,
)
where
    F:
        Send
        + 'static,
{
    let mut total =
        0usize;

    trace_tx
        .send(
            TraceEvent::WorkerStateUpdated {

                kind:
                    WorkerKind::Writer,

                worker_id: 0,

                status:
                    WorkerStatus::Idle,
            }
        )
        .ok();

    for batch in labeled_rx {

        // =========================
        // worker busy
        // =========================
        trace_tx
            .send(
                TraceEvent::WorkerStateUpdated {

                    kind:
                        WorkerKind::Writer,

                    worker_id: 0,

                    status:
                        WorkerStatus::Working {
                            task:
                                "writing".into(),
                        },
                }
            )
            .ok();

        // =========================
        // write
        // =========================
        total +=
            batch.positions.len();

        // TODO:
        // actual dataset write

        // =========================
        // trace
        // =========================
        trace_tx
            .send(
                TraceEvent::Written {
                    games: total,
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
                        WorkerKind::Writer,

                    worker_id: 0,

                    status:
                        WorkerStatus::Idle,
                }
            )
            .ok();
    }
}