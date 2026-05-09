use crate::event::TraceEvent;

pub struct AppState {
    pub total_files: usize,
    pub completed_files: usize,

    pub workers: Vec<WorkerState>,

    pub games_seen: usize,
    pub errors: usize,

    // =========================
    // redraw control
    // =========================
    pub dirty: bool,
}

pub struct WorkerState {
    pub current_file: Option<String>,
}

impl AppState {

    pub fn new(
        total_files: usize,
        num_workers: usize,
    ) -> Self {

        Self {
            total_files,
            completed_files: 0,

            workers: (0..num_workers)
                .map(|_| WorkerState {
                    current_file: None,
                })
                .collect(),

            games_seen: 0,
            errors: 0,

            // initial render
            dirty: true,
        }
    }

    pub fn ingest(
        &mut self,
        event: TraceEvent,
    ) {

        match event {

            // =========================
            // worker/file
            // =========================
            TraceEvent::FileStarted {
                worker_id,
                path,
                ..
            } => {

                self.workers[worker_id]
                    .current_file =
                    Some(path);
            }

            TraceEvent::FileFinished {
                worker_id,
                ..
            } => {

                self.completed_files += 1;

                self.workers[worker_id]
                    .current_file =
                    None;
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

        // =========================
        // mark dirty
        // =========================
        self.dirty = true;
    }
}