use core::position::Position;

use shakmaty::{
    fen::Fen,
    EnPassantMode,
};

use crate::uci::UciEngine;

pub struct EngineEval {

    pub cp: i32,
}

pub struct Stockfish {

    uci: UciEngine,
}

impl Stockfish {

    pub fn new(
        path: &str,
    ) -> Self {

        let mut uci =
            UciEngine::new(path);

        // =====================
        // uci handshake
        // =====================
        uci.send("uci");

        uci.read_until("uciok");

        // =====================
        // ready
        // =====================
        uci.send("isready");

        uci.read_until("readyok");

        Self { uci }
    }

    pub fn evaluate(
        &mut self,
        pos: &Position,
        depth: u32,
    ) -> EngineEval {

        // =====================
        // position
        // =====================
        let fen = Fen::from_position(
            pos.clone(),
            EnPassantMode::Legal,
        )
        .to_string();

        self.uci.send(
            &format!(
                "position fen {}",
                fen,
            )
        );

        // =====================
        // search
        // =====================
        self.uci.send(
            &format!(
                "go depth {}",
                depth,
            )
        );

        let lines =
            self.uci
                .read_until(
                    "bestmove"
                );

        // =====================
        // parse cp
        // =====================
        let mut cp = 0;

        for line in lines {

            if !line.starts_with("info") {
                continue;
            }

            let parts: Vec<_> =
                line
                    .split_whitespace()
                    .collect();

            for i in 0..parts.len() {

                if parts[i] == "cp" {

                    if let Some(v) =
                        parts.get(i + 1)
                    {
                        cp =
                            v.parse()
                                .unwrap_or(0);
                    }
                }
            }
        }

        EngineEval { cp }
    }
}