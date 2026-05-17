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
    Expanded {
        count: usize,
        total_plies: usize,
    },
    Scored { 
        count: usize,
        scores: Vec<f32>,
    },
    Selected { 
        count: usize,
        scores: Vec<f32>,
    },

    // =========================
    // error
    // =========================
    Error {
        stage: &'static str,
    },

    // =========================
    // worker/file
    // =========================
    FileStarted {
        worker_id: usize,
        file_id: u64,
        path: String,
    },

    FileFinished {
        worker_id: usize,
        file_id: u64,
    },
}