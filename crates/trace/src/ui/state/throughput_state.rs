use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};

const THROUGHPUT_WINDOW: Duration =
    Duration::from_secs(5);

pub struct ThroughputSample {
    pub timestamp: Instant,
    pub count: usize,
}

pub struct ThroughputState {

    pub total: usize,

    pub history:
        VecDeque<ThroughputSample>,
}

impl ThroughputState {

    pub fn new() -> Self {

        Self {
            total: 0,
            history: VecDeque::new(),
        }
    }

    pub fn record(
        &mut self,
        count: usize,
    ) {

        self.total += count;

        let now =
            Instant::now();

        self.history.push_back(
            ThroughputSample {
                timestamp: now,
                count,
            }
        );

        while let Some(
            sample
        ) = self.history.front() {

            if now.duration_since(
                sample.timestamp
            ) > THROUGHPUT_WINDOW {

                self.history.pop_front();

            } else {

                break;
            }
        }
    }

    pub fn per_sec(
        &self,
    ) -> f32 {

        if self.history.len() < 2 {
            return 0.0;
        }

        let total: usize =
            self.history
                .iter()
                .map(|s| s.count)
                .sum();

        let oldest =
            self.history
                .front()
                .unwrap()
                .timestamp;

        let newest =
            self.history
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

        total as f32 / elapsed
    }
}