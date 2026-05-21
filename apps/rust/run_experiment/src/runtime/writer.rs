// apps/rust/run_experiment/src/runtime/writer.rs

use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{
    Receiver,
    Sender,
};

use experiment::writer_runner;

use pipeline::types::LabeledBatch;

use trace::event::TraceEvent;

pub fn spawn_writer<F>(

    labeled_rx:
        Receiver<LabeledBatch<F>>,

    trace_tx:
        Sender<TraceEvent>,
)
-> JoinHandle<()>
where
    F:
        Send
        + 'static,
{
    thread::spawn(move || {

        writer_runner::run(

            labeled_rx,

            trace_tx,
        );
    })
}