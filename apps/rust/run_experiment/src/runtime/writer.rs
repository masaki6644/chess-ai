use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::Receiver;

use pipeline::types::LabeledBatch;

pub fn spawn_writer<F>(

    labeled_rx:
        Receiver<LabeledBatch<F>>,
)
-> JoinHandle<()>
where
    F:
        Send
        + 'static,
{
    thread::spawn(move || {

        let mut total =
            0usize;

        for batch
            in labeled_rx
        {
            total +=
                batch.positions.len();


        }
    })
}