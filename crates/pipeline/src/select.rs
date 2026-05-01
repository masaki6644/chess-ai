use crate::score::ScoredPosition;

// =========================
// 出力型
// =========================
#[derive(Clone)]
pub struct SelectedPosition {
    pub pos: ScoredPosition,
}

// =========================
// trait（本体）
// =========================
pub trait Selector {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<SelectedPosition>;
}

// =========================
// デフォルト（全部通す）
// =========================
pub struct NoSelect;

impl Selector for NoSelect {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<SelectedPosition> {
        input
            .into_iter()
            .map(|p| SelectedPosition { pos: p })
            .collect()
    }
}

// =========================
// 例：スコア閾値
// =========================
pub struct ScoreThreshold {
    pub min_score: i32,
}

impl Selector for ScoreThreshold {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<SelectedPosition> {
        input
            .into_iter()
            .filter(|p| p.score >= self.min_score)
            .map(|p| SelectedPosition { pos: p })
            .collect()
    }
}