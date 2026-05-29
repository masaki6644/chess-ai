use crate::labeling::worker::Labeler;

use crate::types::{
    CandidateBatch,
    LabeledBatch,
    LabeledPosition,
    PolicyTarget,
    ValueKind,
};

#[derive(Clone)]
pub struct DummyLabeler;

impl<F> Labeler<F>
    for DummyLabeler
where
    F: Clone,
{
    fn label(
        &mut self,
        batch: CandidateBatch<F>,
    ) -> LabeledBatch<F> {

        let positions =
            batch
                .positions
                .into_iter()
                .map(|candidate| {

                    LabeledPosition {

                        candidate,

                        // =====================
                        // policy target
                        // =====================
                        policy:
                            Vec::<PolicyTarget>::new(),

                        // =====================
                        // value target
                        // =====================
                        value: 0.0,

                        value_kind:
                            ValueKind::Centipawn,
                    }
                })
                .collect();

        LabeledBatch {
            positions,
        }
    }
}