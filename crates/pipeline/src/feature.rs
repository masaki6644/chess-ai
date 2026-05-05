use shakmaty::Position as _;

use crate::types::PositionSample;

// =========================
// Feature trait（差し替え可能な本体）
// =========================
pub trait FeatureBuilder {
    type Output;

    fn build(&self, sample: &PositionSample) -> Self::Output;
}

// =========================
// 最小Feature構造（まずはこれ）
// =========================
#[derive(Clone)]
pub struct SimpleFeatures {
    pub piece_count: u8,
    pub phase: f32,
}

// =========================
// 実装：シンプル版
// =========================
pub struct SimpleFeatureBuilder;

impl FeatureBuilder for SimpleFeatureBuilder {
    type Output = SimpleFeatures;

    fn build(&self, sample: &PositionSample) -> Self::Output {

        // =========================
        // ① 駒数（ここはまだcloneでOK）
        // =========================
        let piece_count: u8 = sample.pos.board().clone().into_iter().count() as u8;

        // =========================
        // ② 進行度（★修正ポイント）
        // =========================
        let phase = if sample.total_plies > 0 {
            sample.ply as f32 / sample.total_plies as f32
        } else {
            0.0
        };

        SimpleFeatures {
            piece_count,
            phase,
        }
    }
}