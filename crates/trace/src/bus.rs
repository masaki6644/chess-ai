use crossbeam_channel::{Receiver};
use crate::event::TraceEvent;

/// TraceBus = single consumer dispatcher thread
pub struct TraceBus {
    rx: Receiver<TraceEvent>,

    analytics_tx: crossbeam_channel::Sender<TraceEvent>,
}

impl TraceBus {
    pub fn new(
        rx: Receiver<TraceEvent>,

        analytics_tx: crossbeam_channel::Sender<TraceEvent>,
    ) -> Self {
        Self {
            rx,

            analytics_tx,
        }
    }

    pub fn run(self) {
        for event in self.rx {

            // =========================
            // Analytics（安定処理）
            // =========================
            let _ = self.analytics_tx.send(event);
            
        }

        drop(self.analytics_tx);
    }
}