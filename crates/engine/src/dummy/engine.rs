use crate::error::EngineError;
use crate::traits::Engine;
use crate::types::Evaluation;

pub struct DummyEngine;

impl Engine for DummyEngine {

    fn init(
        &mut self,
    ) -> Result<(), EngineError> {

        Ok(())
    }

    fn evaluate(
        &mut self,

        _fen: &str,

        depth: u32,
    ) -> Result<Evaluation, EngineError> {

        Ok(Evaluation {

            cp: Some(0),

            mate: None,

            depth,

            nodes: 0,

            pv: vec![],
        })
    }

    fn quit(
        &mut self,
    ) -> Result<(), EngineError> {

        Ok(())
    }
}