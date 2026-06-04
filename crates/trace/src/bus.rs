use std::io::Write;
use std::net::TcpStream;

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
    monitor_stream: Option<TcpStream>,

    analytics_tx: Sender<TraceEvent>,
}

impl TraceBus {

    pub fn new(

        rx: Receiver<TraceEvent>,

        analytics_tx: Sender<TraceEvent>,
    ) -> Self {

        // =========================
        // monitor connection
        // =========================
        let monitor_stream =
            TcpStream::connect(
                "127.0.0.1:7000",
            )
            .ok();

        Self {

            rx,

            monitor_stream,

            analytics_tx,
        }
    }

    pub fn run(mut self) {

        for event in self.rx {

            // =========================
            // monitor
            // =========================
            if let Some(stream) =
                self.monitor_stream.as_mut()
            {

                let json =
                    serde_json::to_string(
                        &event
                    )
                    .unwrap();

                // monitor落ちても
                // experimentは止めない
                if writeln!(
                    stream,
                    "{}",
                    json,
                )
                .is_err()
                {
                    self.monitor_stream =
                        None;
                }
            }

            // =========================
            // Analytics
            // =========================
            let _ =
                self.analytics_tx
                    .send(event);
        }

        drop(self.analytics_tx);
    }
}