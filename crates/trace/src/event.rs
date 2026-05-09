use crate::meta::GameMeta;
use pipeline::filter::FilterReason;

#[derive(Debug, Clone)]
pub enum TraceEvent {
    // =========================
    // game lifecycle
    // =========================
    GameSeen,
    GameAccepted,

    GameFiltered {
        reason: FilterReason,
        meta: GameMeta,
    },

    // =========================
    // pipeline
    // =========================
    Expanded { count: usize },
    Scored { count: usize },
    Selected { count: usize },

    // =========================
    // error
    // =========================
    Error {
        stage: &'static str,
    },
}