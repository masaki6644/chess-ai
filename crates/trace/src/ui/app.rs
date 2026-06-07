use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};

use crate::event::{
    TraceEvent,
    WorkerKind,
    WorkerStatus,
};

const UTIL_WINDOW: Duration =
    Duration::from_secs(5);

const THROUGHPUT_WINDOW: Duration =
    Duration::from_secs(5);

pub struct AppState {

    // =========================
    // overall
    // =========================
    pub total_files: usize,
    pub completed_files: usize,

    pub games_seen: usize,

    // =========================
    // throughput
    // =========================
    pub parse_throughput:
        ThroughputState,

    pub label_throughput:
        ThroughputState,

    pub write_throughput:
        ThroughputState,

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

pub struct ThroughputSample {
    pub timestamp: Instant,
    pub count: usize,
}

pub struct ThroughputState {

    pub total: usize,

    pub history:
        VecDeque<ThroughputSample>,
}

pub struct WorkerState {
    pub status: WorkerStatus,
}

pub struct QueueSample {
    pub timestamp: Instant,
    pub util: f32,
}

pub struct QueueState {
    pub current: usize,
    pub max: usize,

    pub history: VecDeque<QueueSample>,
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
                ThroughputState {
                    total: 0,

                    history:
                        VecDeque::new(),
                },

            label_throughput:
                ThroughputState {
                    total: 0,

                    history:
                        VecDeque::new(),
                },

            write_throughput:
                ThroughputState {
                    total: 0,

                    history:
                        VecDeque::new(),
                },

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

                history: VecDeque::new(),
            },

            labeled_queue: QueueState {
                current: 0,
                max: 0,

                history: VecDeque::new(),
            },

            written_games: 0,

            errors: 0,

            dirty: true,
        }
    }

    fn update_queue(
        queue: &mut QueueState,
        current: usize,
        max: usize,
    ) {
        queue.current = current;
        queue.max = max;

        if max == 0 {
            return;
        }

        let util =
            current as f32
            / max as f32;

        let now =
            Instant::now();

        queue.history.push_back(
            QueueSample {
                timestamp: now,
                util,
            }
        );

        while let Some(
            sample
        ) = queue.history.front() {

            if now.duration_since(
                sample.timestamp
            ) > UTIL_WINDOW {

                queue.history.pop_front();

            } else {

                break;
            }
        }
    }

    fn record_throughput(
        throughput:
            &mut ThroughputState,

        count: usize,
    ) {

        throughput.total += count;

        let now =
            Instant::now();

        throughput.history.push_back(
            ThroughputSample {
                timestamp: now,
                count,
            }
        );

        while let Some(
            sample
        ) = throughput.history.front() {

            if now.duration_since(
                sample.timestamp
            ) > THROUGHPUT_WINDOW {

                throughput
                    .history
                    .pop_front();

            } else {

                break;
            }
        }
    }

    pub fn throughput_per_sec(
        throughput:
            &ThroughputState,
    ) -> f32 {

        if throughput
            .history
            .len()
            < 2
        {
            return 0.0;
        }

        let total: usize =
            throughput
                .history
                .iter()
                .map(|s| s.count)
                .sum();

        let oldest =
            throughput
                .history
                .front()
                .unwrap()
                .timestamp;

        let newest =
            throughput
                .history
                .back()
                .unwrap()
                .timestamp;

        let elapsed =
            newest
                .duration_since(
                    oldest,
                )
                .as_secs_f32();

        if elapsed <= 0.0 {

            return 0.0;
        }

        total as f32
        /
        elapsed
    }

    pub fn queue_avg(
        queue: &QueueState,
    ) -> f32 {

        if queue.history.is_empty() {
            return 0.0;
        }

        queue.history
            .iter()
            .map(|s| s.util)
            .sum::<f32>()
            /
            queue.history.len() as f32
    }

    pub fn queue_peak(
        queue: &QueueState,
    ) -> f32 {

        queue.history
            .iter()
            .map(|s| s.util)
            .fold(
                0.0,
                f32::max,
            )
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

                Self::update_queue(
                    &mut self.candidate_queue,
                    current,
                    max,
                );
            }

            TraceEvent::LabeledQueue {
                current,
                max,
            } => {

                Self::update_queue(
                    &mut self.labeled_queue,
                    current,
                    max,
                );
            }

            // =========================
            // throughput
            // =========================
            TraceEvent::GameSeen => {

                self.games_seen += 1;

                Self::record_throughput(
                    &mut self.parse_throughput,
                    1,
                );
            }

            TraceEvent::Selected {
                count,
                ..
            } => {

                Self::record_throughput(
                    &mut self.label_throughput,
                    count,
                );
            }

            TraceEvent::Written {
                games,
            } => {

                let delta =
                    games.saturating_sub(
                        self.written_games,
                    );

                self.written_games =
                    games;

                Self::record_throughput(
                    &mut self.write_throughput,
                    delta,
                );
            }

            // =========================
            // errors
            // =========================
            TraceEvent::Error { .. } => {
                self.errors += 1;
            }

            _ => {}
        }

        self.dirty = true;
    }
}