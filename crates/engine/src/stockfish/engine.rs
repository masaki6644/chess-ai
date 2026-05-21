use crate::error::EngineError;
use crate::stockfish::config::StockfishConfig;
use crate::stockfish::parser::parse_info_line;
use crate::stockfish::process::StockfishProcess;
use crate::traits::Engine;
use crate::types::Evaluation;

pub struct StockfishEngine {

    pub config:
        StockfishConfig,

    pub process:
        StockfishProcess,
}

impl StockfishEngine {

    pub fn new(
        config: StockfishConfig,
    ) -> Result<Self, EngineError> {

        let process =
            StockfishProcess::spawn(
                &config.path,
            )?;

        Ok(Self {
            config,
            process,
        })
    }

    fn wait_for(
        &mut self,
        target: &str,
    ) -> Result<(), EngineError> {

        loop {

            let line =
                self.process
                    .read_line()?;

            if line.trim() == target {
                return Ok(());
            }
        }
    }
}

impl Engine for StockfishEngine {

    fn init(
        &mut self,
    ) -> Result<(), EngineError> {

        self.process
            .write_line("uci")?;

        self.wait_for("uciok")?;

        self.process
            .write_line(&format!(
                "setoption name Threads value {}",
                self.config.threads,
            ))?;

        self.process
            .write_line(&format!(
                "setoption name Hash value {}",
                self.config.hash_mb,
            ))?;

        self.process
            .write_line("isready")?;

        self.wait_for("readyok")?;

        Ok(())
    }

    fn evaluate(
        &mut self,

        fen: &str,

        depth: u32,
    ) -> Result<Evaluation, EngineError> {

        self.process
            .write_line(
                &format!(
                    "position fen {}",
                    fen,
                )
            )?;

        self.process
            .write_line(
                &format!(
                    "go depth {}",
                    depth,
                )
            )?;

        let mut latest = None;

        loop {

            let line =
                self.process
                    .read_line()?;

            if line.starts_with(
                "info"
            ) {

                if let Some(eval) =
                    parse_info_line(
                        &line
                    )
                {
                    latest =
                        Some(eval);
                }
            }

            if line.starts_with(
                "bestmove"
            ) {
                break;
            }
        }

        latest.ok_or_else(|| {
            EngineError::Protocol(
                "no evaluation".into()
            )
        })
    }

    fn quit(
        &mut self,
    ) -> Result<(), EngineError> {

        self.process
            .write_line("quit")?;

        Ok(())
    }
}