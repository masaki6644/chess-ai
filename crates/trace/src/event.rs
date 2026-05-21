use crate::meta::GameMeta;
use pipeline::candidate::filter::FilterReason;

#[derive(Debug, Clone)]
pub enum WorkerKind {
    Parse,
    Label,
    Writer,
}

#[derive(Debug, Clone)]
pub enum WorkerStatus {

    Idle,

    Working {
        task: String,
    },
}

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
    // queue
    // =========================
    CandidateQueue {
        current: usize,
        max: usize,
    },

    LabeledQueue {
        current: usize,
        max: usize,
    },

    // =========================
    // writer
    // =========================
    Written {
        games: usize,
    },

    // =========================
    // workers
    // =========================
    WorkerStateUpdated {

        kind: WorkerKind,

        worker_id: usize,

        status: WorkerStatus,
    },

    // =========================
    // parser/file
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