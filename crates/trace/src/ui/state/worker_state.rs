use crate::event::WorkerStatus;

pub struct WorkerState {
    pub status: WorkerStatus,
}

impl WorkerState {

    pub fn idle() -> Self {

        Self {
            status: WorkerStatus::Idle,
        }
    }
}