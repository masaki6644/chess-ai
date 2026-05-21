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

        Self {

            parse_workers:
                base
                    .saturating_sub(4)
                    .max(1),

            label_workers: 2,

            candidate_queue_size: 1024,

            labeled_queue_size: 1024,
        }
    }
}