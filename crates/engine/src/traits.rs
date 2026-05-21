use crate::error::EngineError;
use crate::types::Evaluation;

pub trait Engine:
    Send
{
    fn init(
        &mut self,
    ) -> Result<(), EngineError>;

    fn evaluate(
        &mut self,

        fen: &str,

        depth: u32,
    ) -> Result<Evaluation, EngineError>;

    fn quit(
        &mut self,
    ) -> Result<(), EngineError>;
}