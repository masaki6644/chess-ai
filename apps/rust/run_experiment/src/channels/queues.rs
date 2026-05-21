use crossbeam::channel::{
    bounded,
    Receiver,
    Sender,
};

use pipeline::types::{
    CandidateBatch,
    LabeledBatch,
};

// =========================
// pipeline queues
// =========================
pub struct PipelineQueues<F> {

    pub candidate_tx:
        Sender<CandidateBatch<F>>,

    pub candidate_rx:
        Receiver<CandidateBatch<F>>,

    pub labeled_tx:
        Sender<LabeledBatch<F>>,

    pub labeled_rx:
        Receiver<LabeledBatch<F>>,
}

impl<F> PipelineQueues<F> {

    pub fn new(

        candidate_capacity:
            usize,

        labeled_capacity:
            usize,
    ) -> Self {

        let (
            candidate_tx,
            candidate_rx,
        ) = bounded(
            candidate_capacity
        );

        let (
            labeled_tx,
            labeled_rx,
        ) = bounded(
            labeled_capacity
        );

        Self {

            candidate_tx,
            candidate_rx,

            labeled_tx,
            labeled_rx,
        }
    }
}