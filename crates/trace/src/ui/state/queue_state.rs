use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};

const UTIL_WINDOW: Duration =
    Duration::from_secs(5);

pub struct QueueSample {
    pub timestamp: Instant,
    pub util: f32,
}

pub struct QueueState {

    pub current: usize,
    pub max: usize,

    pub history:
        VecDeque<QueueSample>,
}

impl QueueState {

    pub fn new() -> Self {

        Self {
            current: 0,
            max: 0,
            history: VecDeque::new(),
        }
    }

    pub fn update(
        &mut self,
        current: usize,
        max: usize,
    ) {

        self.current = current;
        self.max = max;

        if max == 0 {
            return;
        }

        let util =
            current as f32
            / max as f32;

        let now =
            Instant::now();

        self.history.push_back(
            QueueSample {
                timestamp: now,
                util,
            }
        );

        while let Some(
            sample
        ) = self.history.front() {

            if now.duration_since(
                sample.timestamp
            ) > UTIL_WINDOW {

                self.history.pop_front();

            } else {

                break;
            }
        }
    }

    pub fn avg(
        &self,
    ) -> f32 {

        if self.history.is_empty() {
            return 0.0;
        }

        self.history
            .iter()
            .map(|s| s.util)
            .sum::<f32>()
            /
            self.history.len() as f32
    }

    pub fn peak(
        &self,
    ) -> f32 {

        self.history
            .iter()
            .map(|s| s.util)
            .fold(
                0.0,
                f32::max,
            )
    }
}