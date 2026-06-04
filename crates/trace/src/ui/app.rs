use std::time::Instant;

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

    pub candidate_util_avg: f64,
    pub labeled_util_avg: f64,

    pub candidate_samples: usize,
    pub labeled_samples: usize,

    // =========================
    // writer
    // =========================
    pub written_games: usize,

    pub write_rate: f64,
    pub last_written_games: usize,

    pub last_rate_update: Instant,

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

            candidate_util_avg: 0.0,
            labeled_util_avg: 0.0,

            candidate_samples: 0,
            labeled_samples: 0,

            written_games: 0,

            write_rate: 0.0,
            last_written_games: 0,

            last_rate_update: Instant::now(),

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
            // parser
            // =========================
            TraceEvent::FileStarted {
                worker_id,
                path,
                ..
            } => {

                if let Some(worker) =
                    self.parse_workers.get_mut(worker_id)
                {
                    worker.status =
                        WorkerStatus::Working {
                            task: path,
                        };
                }
            }

            TraceEvent::FileFinished {
                worker_id,
                ..
            } => {

                self.completed_files += 1;

                if let Some(worker) =
                    self.parse_workers.get_mut(worker_id)
                {
                    worker.status =
                        WorkerStatus::Idle;
                }
            }

            // =========================
            // workers
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

                if let Some(worker) =
                    workers.get_mut(worker_id)
                {
                    worker.status =
                        status;
                }
            }

            // =========================
            // candidate queue
            // =========================
            TraceEvent::CandidateQueue {
                current,
                max,
            } => {

                self.candidate_queue.current =
                    current;

                self.candidate_queue.max =
                    max;

                if max > 0 {

                    let util =
                        current as f64
                        / max as f64;

                    self.candidate_util_avg =
                        (
                            self.candidate_util_avg
                            * self.candidate_samples as f64
                            + util
                        )
                        /
                        (
                            self.candidate_samples + 1
                        ) as f64;

                    self.candidate_samples += 1;
                }
            }

            // =========================
            // labeled queue
            // =========================
            TraceEvent::LabeledQueue {
                current,
                max,
            } => {

                self.labeled_queue.current =
                    current;

                self.labeled_queue.max =
                    max;

                if max > 0 {

                    let util =
                        current as f64
                        / max as f64;

                    self.labeled_util_avg =
                        (
                            self.labeled_util_avg
                            * self.labeled_samples as f64
                            + util
                        )
                        /
                        (
                            self.labeled_samples + 1
                        ) as f64;

                    self.labeled_samples += 1;
                }
            }

            // =========================
            // writer
            // =========================
            TraceEvent::Written {
                games,
            } => {

                let now =
                    Instant::now();

                let elapsed =
                    now.duration_since(
                        self.last_rate_update,
                    );

                self.written_games =
                    games;

                if elapsed.as_secs_f64() >= 5.0 {

                    let diff =
                        self.written_games
                            .saturating_sub(
                                self.last_written_games,
                            );

                    self.write_rate =
                        diff as f64
                        / elapsed.as_secs_f64();

                    self.last_written_games =
                        self.written_games;

                    self.last_rate_update =
                        now;
                }
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