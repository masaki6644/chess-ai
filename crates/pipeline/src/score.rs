use core::Position;

#[derive(Clone)]
pub struct ScoredPosition {
    pub pos: Position,
    pub score: i32,
}

// =========================
// trait（差し替え可能にする本体）
// =========================
pub trait Scorer {
    fn score(&self, pos: Position) -> ScoredPosition;
}

// =========================
// デフォルト実装（今までのやつ）
// =========================
pub struct DummyScorer;

impl Scorer for DummyScorer {
    fn score(&self, pos: Position) -> ScoredPosition {
        ScoredPosition {
            pos,
            score: 0,
        }
    }
}