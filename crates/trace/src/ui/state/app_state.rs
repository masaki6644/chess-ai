use super::{
    queue_state::QueueState,
    throughput_state::ThroughputState,
    worker_state::WorkerState,
};

pub struct AppState {

    pub total_files: usize,
    pub completed_files: usize,

    pub games_seen: usize,

    pub parse_throughput:
        ThroughputState,

    pub label_throughput:
        ThroughputState,

    pub write_throughput:
        ThroughputState,

    pub parse_workers:
        Vec<WorkerState>,

    pub label_workers:
        Vec<WorkerState>,

    pub writer_workers:
        Vec<WorkerState>,

    pub candidate_queue:
        QueueState,

    pub labeled_queue:
        QueueState,

    pub written_games: usize,

    pub errors: usize,

    pub dirty: bool,
}

impl AppState {

    pub fn new(
        total_files: usize,
        num_parse_workers: usize,
        num_label_workers: usize,
    ) -> Self {

        Self {

            total_files,
            completed_files: 0,

            games_seen: 0,

            parse_throughput:
                ThroughputState::new(),

            label_throughput:
                ThroughputState::new(),

            write_throughput:
                ThroughputState::new(),

            parse_workers:
                (0..num_parse_workers)
                .map(|_| WorkerState::idle())
                .collect(),

            label_workers:
                (0..num_label_workers)
                .map(|_| WorkerState::idle())
                .collect(),

            writer_workers:
                vec![
                    WorkerState::idle()
                ],

            candidate_queue:
                QueueState::new(),

            labeled_queue:
                QueueState::new(),

            written_games: 0,

            errors: 0,

            dirty: true,
        }
    }
}