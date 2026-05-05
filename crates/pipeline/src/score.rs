use core::Position;

use crate::types::PositionSample;
use crate::feature::SimpleFeatures;

#[derive(Clone)]
pub struct ScoredPosition {
    pub pos: Position,
    pub score: f32,
}

// =========================
// trait（Feature前提）
// =========================
pub trait Scorer<F>: Send + Sync  {
    fn score(&self, sample: PositionSample, features: F) -> ScoredPosition;
}

// =========================
// Dummy
// =========================
pub struct DummyScorer;

impl<F> Scorer<F> for DummyScorer {
    fn score(&self, sample: PositionSample, _features: F) -> ScoredPosition {
        ScoredPosition {
            pos: sample.pos,
            score: 0.0,
        }
    }
}

// =========================
// QuickScorer（完成版）
// =========================
pub struct QuickScorer;

impl Scorer<SimpleFeatures> for QuickScorer {
    fn score(&self, sample: PositionSample, f: SimpleFeatures) -> ScoredPosition {

        // =========================
        // ① density
        // =========================
        let density = f.piece_count as f32 / 32.0;

        // =========================
        // ② variance
        // =========================
        let variance = density * (1.0 - density);

        // =========================
        // ③ phase（連続値そのまま使う）
        // =========================
        // opening寄り → 小さく
        // endgame寄り → やや下げる
        let phase_bias = 
            1.0 - (f.phase - 0.5).abs();  
        // ↑ 中盤(0.5)で最大になる形

        // =========================
        // ④ score
        // =========================
        let score = 0.6 * variance + 0.4 * phase_bias;

        ScoredPosition {
            pos: sample.pos,
            score,
        }
    }
}