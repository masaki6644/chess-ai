use crate::event::{
    TraceEvent,
    WorkerKind,
    WorkerStatus,
};

use super::AppState;

impl AppState {

    pub fn ingest(
        &mut self,
        event: TraceEvent,
    ) {

        match event {

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

            TraceEvent::CandidateQueue {
                current,
                max,
            } => {

                self.candidate_queue
                    .update(
                        current,
                        max,
                    );
            }

            TraceEvent::LabeledQueue {
                current,
                max,
            } => {

                self.labeled_queue
                    .update(
                        current,
                        max,
                    );
            }

            TraceEvent::GameSeen => {

                self.games_seen += 1;

                self.parse_throughput
                    .record(1);
            }

            TraceEvent::Selected {
                count,
                ..
            } => {

                self.label_throughput
                    .record(count);
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

                self.write_throughput
                    .record(delta);
            }

            TraceEvent::Error { .. } => {
                self.errors += 1;
            }

            _ => {}
        }

        self.dirty = true;
    }
}