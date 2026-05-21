use crate::candidate::score::ScoredPosition;

use rand::prelude::*;

// =========================
// trait
// =========================
pub trait Selector<F>:
    Send + Sync
{
    fn select(
        &self,
        input: Vec<ScoredPosition<F>>,
    ) -> Vec<ScoredPosition<F>>;
}

// =========================
// デフォルト（全部通す）
// =========================
pub struct NoSelect;

impl<F> Selector<F> for NoSelect {

    fn select(
        &self,
        input: Vec<ScoredPosition<F>>,
    ) -> Vec<ScoredPosition<F>> {

        input
    }
}

// =========================
// スコア閾値
// =========================
pub struct ScoreThreshold {
    pub min_score: f32,
}

impl<F> Selector<F>
    for ScoreThreshold
{
    fn select(
        &self,
        input: Vec<ScoredPosition<F>>,
    ) -> Vec<ScoredPosition<F>> {

        input
            .into_iter()
            .filter(|p| {
                p.score >= self.min_score
            })
            .collect()
    }
}

// =========================
// Top-K Selector
// =========================
pub struct TopKSelector {
    pub k: usize,
    pub min_score: f32,
}

impl<F> Selector<F>
    for TopKSelector
{
    fn select(
        &self,
        mut input: Vec<ScoredPosition<F>>,
    ) -> Vec<ScoredPosition<F>> {

        if input.is_empty() {
            return vec![];
        }

        // =========================
        // score desc
        // =========================
        input.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap()
        });

        // =========================
        // threshold + topk
        // =========================
        input
            .into_iter()
            .filter(|p| {
                p.score >= self.min_score
            })
            .take(self.k)
            .collect()
    }
}

// =========================
// Soft Selector
// =========================
pub struct SoftSelector {
    pub temperature: f32,
    pub k: usize,
}

impl<F> Selector<F>
    for SoftSelector
{
    fn select(
        &self,
        input: Vec<ScoredPosition<F>>,
    ) -> Vec<ScoredPosition<F>> {

        if input.is_empty()
            || self.k == 0
        {
            return vec![];
        }

        let mut rng = thread_rng();

        // =========================
        // softmax weights
        // =========================
        let mut weights: Vec<f32> =
            input
                .iter()
                .map(|p| {
                    (
                        p.score
                        / self.temperature
                    ).exp()
                })
                .collect();

        let sum: f32 =
            weights.iter().sum();

        if sum == 0.0 {
            return vec![];
        }

        for w in &mut weights {
            *w /= sum;
        }

        // =========================
        // roulette selection
        // =========================
        let mut selected =
            Vec::with_capacity(self.k);

        let mut candidates = input;

        for _ in 0..self.k {

            if candidates.is_empty() {
                break;
            }

            let r: f32 = rng.gen();

            let mut acc = 0.0;

            let mut idx = 0usize;

            for (i, w)
                in weights.iter().enumerate()
            {
                acc += w;

                if r <= acc {
                    idx = i;
                    break;
                }
            }

            selected.push(
                candidates.remove(idx)
            );

            weights.remove(idx);

            // =========================
            // renormalize
            // =========================
            let sum: f32 =
                weights.iter().sum();

            if sum > 0.0 {

                for w in &mut weights {
                    *w /= sum;
                }
            }
        }

        selected
    }
}