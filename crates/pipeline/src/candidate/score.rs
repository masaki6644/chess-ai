
use crate::candidate::types::PositionSample;
use crate::candidate::feature::SimpleFeatures;

// =========================
// ScoredPosition
// =========================
#[derive(Clone)]
pub struct ScoredPosition<F> {

    pub sample: PositionSample,

    pub features: F,

    pub score: f32,
}

// =========================
// trait
// =========================
pub trait Scorer<F>:
    Send + Sync
{
    fn score(
        &self,

        sample: PositionSample,

        features: F,
    ) -> ScoredPosition<F>;
}

// =========================
// Dummy
// =========================
pub struct DummyScorer;

impl<F> Scorer<F>
    for DummyScorer
where
    F: Clone,
{
    fn score(
        &self,

        sample: PositionSample,

        features: F,
    ) -> ScoredPosition<F> {

        ScoredPosition {

            sample,

            features,

            score: 0.0,
        }
    }
}

// =========================
// QuickScorer
// =========================
pub struct QuickScorer;

impl Scorer<SimpleFeatures>
    for QuickScorer
{
    fn score(
        &self,

        sample: PositionSample,

        f: SimpleFeatures,
    ) -> ScoredPosition<SimpleFeatures> {

        // =========================
        // density
        // =========================
        let density =
            f.piece_count as f32 / 32.0;

        // =========================
        // variance
        // =========================
        let variance =
            density * (1.0 - density);

        // =========================
        // phase bias
        // =========================
        let phase_bias =
            1.0 - (f.phase - 0.5).abs();

        // =========================
        // score
        // =========================
        let score =
            0.6 * variance
            + 0.4 * phase_bias;

        ScoredPosition {

            sample,

            features: f,

            score,
        }
    }
}