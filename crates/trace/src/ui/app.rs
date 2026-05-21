use crate::event::{
    TraceEvent,
    WorkerKind,
    WorkerStatus,
};

pub struct AppState {

    // =========================
    // overall
    // =========================
    pub total_files: usize,
    pub completed_files: usize,

    pub games_seen: usize,

    // =========================
    // workers
    // =========================
    pub parse_workers: Vec<WorkerState>,
    pub label_workers: Vec<WorkerState>,
    pub writer_workers: Vec<WorkerState>,

    // =========================
    // queues
    // =========================
    pub candidate_queue: QueueState,
    pub labeled_queue: QueueState,


    pub written_games: usize,

    // =========================
    // errors
    // =========================
    pub errors: usize,

    // =========================
    // redraw
    // =========================
    pub dirty: bool,
}

pub struct WorkerState {
    pub status: WorkerStatus,
}

pub struct QueueState {
    pub current: usize,
    pub max: usize,
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

            parse_workers:
                (0..num_parse_workers)
                .map(|_| WorkerState {
                    status: WorkerStatus::Idle,
                })
                .collect(),

            label_workers:
                (0..num_label_workers)
                .map(|_| WorkerState {
                    status: WorkerStatus::Idle,
                })
                .collect(),

            writer_workers:
                vec![
                    WorkerState {
                        status: WorkerStatus::Idle,
                    }
                ],

            candidate_queue: QueueState {
                current: 0,
                max: 0,
            },

            labeled_queue: QueueState {
                current: 0,
                max: 0,
            },

            written_games: 0,

            errors: 0,

            dirty: true,
        }
    }

    pub fn ingest(
        &mut self,
        event: TraceEvent,
    ) {

        match event {

            // =========================
            // parser files
            // =========================
            TraceEvent::FileStarted {
                worker_id,
                path,
                ..
            } => {

                self.parse_workers[worker_id]
                    .status =
                    WorkerStatus::Working {
                        task: path,
                    };
            }

            TraceEvent::FileFinished {
                worker_id,
                ..
            } => {

                self.completed_files += 1;

                self.parse_workers[worker_id]
                    .status =
                    WorkerStatus::Idle;
            }

            // =========================
            // worker state
            // =========================
            TraceEvent::WorkerStateUpdated {

                kind,
                worker_id,
                status,

            } => {

                let workers =
                    match kind {

                        WorkerKind::Parse =>
                            &mut self.parse_workers,

                        WorkerKind::Label =>
                            &mut self.label_workers,

                        WorkerKind::Writer =>
                            &mut self.writer_workers,
                    };

                workers[worker_id]
                    .status = status;
            }

            // =========================
            // queues
            // =========================
            TraceEvent::CandidateQueue {
                current,
                max,
            } => {

                self.candidate_queue.current =
                    current;

                self.candidate_queue.max =
                    max;
            }

            TraceEvent::LabeledQueue {
                current,
                max,
            } => {

                self.labeled_queue.current =
                    current;

                self.labeled_queue.max =
                    max;
            }

            TraceEvent::Written {
                games,
            } => {

                self.written_games =
                    games;
            }

            // =========================
            // counters
            // =========================
            TraceEvent::GameSeen => {
                self.games_seen += 1;
            }

            TraceEvent::Error { .. } => {
                self.errors += 1;
            }

            _ => {}
        }

        self.dirty = true;
    }
}