use engine::traits::Engine;
use search::mcts::search::run_mcts;

use crate::labeling::worker::Labeler;

use crate::types::{
    CandidateBatch,
    LabeledBatch,
    LabeledPosition,
    PolicyTarget,
    ValueKind,
};

pub struct MctsLabeler<E> {
    pub engine: E,

    pub simulations: usize,
    pub depth: u32,
    pub c_puct: f32,
}

impl<E, F> Labeler<F> for MctsLabeler<E>
where
    E: Engine + Send + 'static,
    F: Clone,
{
    fn label(
        &mut self,
        batch: CandidateBatch<F>,
    ) -> LabeledBatch<F> {

        let mut positions = Vec::new();

        for candidate in batch.positions {

            let result = run_mcts(
                &mut self.engine,
                &candidate.fen,
                self.simulations,
                self.depth,
                self.c_puct,
            );

            let policy = result
                .policy
                .into_iter()
                .map(|p| PolicyTarget {
                    mv: p.mv,
                    visits: p.visits,
                    prob: p.probability,
                })
                .collect();

            positions.push(LabeledPosition {
                candidate,
                policy,
                value: result.value,
                value_kind: ValueKind::WinRate,
            });
        }

        LabeledBatch { positions }
    }
}