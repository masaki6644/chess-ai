use crossbeam_channel::{
    Receiver,
    Sender,
};

use crate::event::TraceEvent;

/// TraceBus = single consumer dispatcher thread
pub struct TraceBus {
    rx: Receiver<TraceEvent>,

    // =========================
    // consumers
    // =========================
    ui_tx: Sender<TraceEvent>,
    analytics_tx: Sender<TraceEvent>,
}

impl TraceBus {

    pub fn new(
        rx: Receiver<TraceEvent>,

        ui_tx: Sender<TraceEvent>,
        analytics_tx: Sender<TraceEvent>,
    ) -> Self {

        Self {
            rx,

            ui_tx,
            analytics_tx,
        }
    }

    pub fn run(self) {

        for event in self.rx {

            // =========================
            // UI（drop許容）
            // =========================
            let _ =
                self.ui_tx.try_send(event.clone());

            // =========================
            // Analytics（安定処理）
            // =========================
            let _ =
                self.analytics_tx.send(event);
        }

        drop(self.ui_tx);
        drop(self.analytics_tx);
    }
}