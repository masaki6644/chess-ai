use engine::stockfish::config::StockfishConfig;
use engine::stockfish::engine::StockfishEngine;
use engine::traits::Engine;

use crate::labeling::factory::LabelerFactory;
use crate::labeling::mcts::MctsLabeler;

pub struct MctsLabelerFactory {
    pub stockfish: StockfishConfig,

    pub simulations: usize,

    pub depth: u32,

    pub c_puct: f32,
}

impl<F> LabelerFactory<F>
    for MctsLabelerFactory
where
    F: Clone,
{
    type LabelerType =
        MctsLabeler<StockfishEngine>;

    fn create(
        &self,
        _worker_id: usize,
    )
    -> Self::LabelerType
    {
        let mut engine =
            StockfishEngine::new(
                self.stockfish.clone(),
            )
            .unwrap();

        engine.init().unwrap();

        MctsLabeler {

            engine,

            simulations:
                self.simulations,

            depth:
                self.depth,

            c_puct:
                self.c_puct,
        }
    }
}