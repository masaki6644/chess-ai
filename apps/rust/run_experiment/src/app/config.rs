#[derive(Debug, Clone)]
pub struct AppConfig {

    // =========================
    // parse
    // =========================
    pub parse_workers: usize,

    // =========================
    // labeling
    // =========================
    pub label_workers: usize,

    // =========================
    // queues
    // =========================
    pub candidate_queue_size: usize,

    pub labeled_queue_size: usize,
}

impl AppConfig {

    pub fn default() -> Self {

        let base =
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4);

        let label_workers =
            (base / 3).max(2);

        let parse_workers =
            (base - label_workers - 1)
                .max(1);

        Self {

            parse_workers,

            label_workers,

            candidate_queue_size: 1024,

            labeled_queue_size: 1024,
        }
    }
}