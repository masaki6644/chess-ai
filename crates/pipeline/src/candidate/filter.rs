use core::Game;

use serde::{
    Serialize,
    Deserialize,
};

// =========================
// 理由（観測用）
// =========================
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum FilterReason {
    TooShort,
    TooLong,
    NoResult,
    NoElo,
    EloOutOfRange,
}

// =========================
// trait（Resultベース）
// =========================
pub trait GameFilter: Send + Sync {
    fn check(&self, game: &Game) -> Result<(), FilterReason>;
}

// =========================
// NoFilter
// =========================
pub struct NoFilter;

impl GameFilter for NoFilter {
    fn check(&self, _game: &Game) -> Result<(), FilterReason> {
        Ok(())
    }
}

// =========================
// Config（重要）
// =========================
#[derive(Debug, Clone, Copy)]
pub struct StrongGameFilterConfig {
    pub min_len: usize,
    pub max_len: usize,
    pub min_elo: u32,
    pub max_elo: u32,
}

// =========================
// 本体
// =========================
pub struct StrongGameFilter {
    pub config: StrongGameFilterConfig,
}

impl GameFilter for StrongGameFilter {
    fn check(&self, game: &Game) -> Result<(), FilterReason> {
        let len = game.moves.len();

        // ① 長さ
        if len < self.config.min_len {
            return Err(FilterReason::TooShort);
        }

        if len > self.config.max_len {
            return Err(FilterReason::TooLong);
        }

        // ② 結果
        game.result.ok_or(FilterReason::NoResult)?;

        // ③ Elo
        let (w, b) = match (game.white_elo, game.black_elo) {
            (Some(w), Some(b)) => (w, b),
            _ => return Err(FilterReason::NoElo),
        };

        let avg = (w + b) / 2;

        if avg < self.config.min_elo || avg > self.config.max_elo {
            return Err(FilterReason::EloOutOfRange);
        }

        Ok(())
    }
}