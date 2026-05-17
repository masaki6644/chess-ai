use core::Position;

use crate::score::ScoredPosition;

// =========================
// labeled
// =========================
#[derive(Clone)]
pub struct LabeledPosition {

    pub pos: Position,

    // centipawn eval
    pub cp: i32,
}

// =========================
// trait
// =========================
pub trait Labeler: Send + Sync {

    fn label(
        &self,
        positions: Vec<ScoredPosition>,
    ) -> Vec<LabeledPosition>;
}

// =========================
// dummy
// =========================
pub struct DummyLabeler;

impl Labeler for DummyLabeler {

    fn label(
        &self,
        positions: Vec<ScoredPosition>,
    ) -> Vec<LabeledPosition> {

        positions
            .into_iter()
            .map(|p| {

                LabeledPosition {

                    pos: p.pos,

                    // dummy
                    cp: 0,
                }
            })
            .collect()
    }
}