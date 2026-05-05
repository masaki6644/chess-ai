use crate::score::ScoredPosition;
use rand::prelude::*;

// =========================
// trait
// =========================
pub trait Selector: Send + Sync  {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<ScoredPosition>;
}

// =========================
// デフォルト（全部通す）
// =========================
pub struct NoSelect;

impl Selector for NoSelect {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<ScoredPosition> {
        input
    }
}

// =========================
// スコア閾値
// =========================
pub struct ScoreThreshold {
    pub min_score: f32,
}

impl Selector for ScoreThreshold {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<ScoredPosition> {
        input
            .into_iter()
            .filter(|p| p.score >= self.min_score)
            .collect()
    }
}

// =========================
// Top-K Selector（実用版）
// =========================
pub struct TopKSelector {
    pub k: usize,
    pub min_score: f32,
}

impl Selector for TopKSelector {
    fn select(&self, mut input: Vec<ScoredPosition>) -> Vec<ScoredPosition> {
        if input.is_empty() {
            return vec![];
        }

        // ① スコア降順ソート
        input.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // ② 閾値フィルタ + top-k
        input
            .into_iter()
            .filter(|p| p.score >= self.min_score)
            .take(self.k)
            .collect()
    }
}


// =========================
// Soft (Temperature Sampling)
// =========================
pub struct SoftSelector {
    pub temperature: f32,
    pub k: usize,
}

impl Selector for SoftSelector {
    fn select(&self, input: Vec<ScoredPosition>) -> Vec<ScoredPosition> {
        if input.is_empty() || self.k == 0 {
            return vec![];
        }

        let mut rng = thread_rng();

        // -------------------------
        // ① softmax重み計算
        // -------------------------
        let mut weights: Vec<f32> = input
            .iter()
            .map(|p| (p.score / self.temperature).exp())
            .collect();

        let sum: f32 = weights.iter().sum();

        if sum == 0.0 {
            return vec![];
        }

        for w in &mut weights {
            *w /= sum;
        }

        // -------------------------
        // ② ルーレット選択
        // -------------------------
        let mut selected = Vec::with_capacity(self.k);

        let mut candidates = input;

        for _ in 0..self.k {
            if candidates.is_empty() {
                break;
            }

            let mut acc = 0.0;
            let r: f32 = rng.gen();

            let mut idx = 0;

            for (i, w) in weights.iter().enumerate() {
                acc += w;
                if r <= acc {
                    idx = i;
                    break;
                }
            }

            selected.push(candidates.remove(idx));
            weights.remove(idx);

            // 再正規化
            let sum: f32 = weights.iter().sum();
            if sum > 0.0 {
                for w in &mut weights {
                    *w /= sum;
                }
            }
        }

        selected
    }
}