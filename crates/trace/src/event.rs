use crate::meta::GameMeta;
use pipeline::filter::FilterReason;

#[derive(Debug, Clone)]
pub enum TraceEvent {
    GameSeen,
    GameAccepted,

    GameFiltered {
        reason: FilterReason,
        meta: GameMeta,
    },

    Expanded { count: usize },
    Scored { count: usize },
    Selected { count: usize },

    Error {
        stage: &'static str,
    },
}