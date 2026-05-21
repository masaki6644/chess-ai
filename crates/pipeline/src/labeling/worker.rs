use crossbeam::channel::{
    Receiver,
    Sender,
};

use crate::types::{
    CandidateBatch,
    LabeledBatch,
};

// =========================
// Labeler trait
// =========================
pub trait Labeler<F>:
    Send + Sync + 'static
{
    fn label(
        &self,
        batch: CandidateBatch<F>,
    ) -> LabeledBatch<F>;
}

// =========================
// labeling worker loop
// =========================
pub fn run_labeling_worker<F, L>(

    candidate_rx:
        Receiver<CandidateBatch<F>>,

    labeled_tx:
        Sender<LabeledBatch<F>>,

    labeler: L,
)
where
    F: Send + 'static,

    L: Labeler<F>,
{
    for batch in candidate_rx {

        let labeled =
            labeler.label(batch);

        labeled_tx
            .send(labeled)
            .expect(
                "failed to send labeled batch"
            );
    }
}