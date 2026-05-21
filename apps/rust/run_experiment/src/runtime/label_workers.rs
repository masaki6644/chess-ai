use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{
    Receiver,
    Sender,
};

use pipeline::labeling::worker::Labeler;

use pipeline::types::{
    CandidateBatch,
    LabeledBatch,
};

pub fn spawn_label_workers<F, L>(

    num_workers: usize,

    candidate_rx:
        Receiver<CandidateBatch<F>>,

    labeled_tx:
        Sender<LabeledBatch<F>>,

    labeler: L,
)
-> Vec<JoinHandle<()>>
where
    F:
        Send
        + 'static,

    L:
        Labeler<F>
        + Clone,
{
    let mut handles =
        Vec::new();

    for _ in 0..num_workers {

        let candidate_rx =
            candidate_rx.clone();

        let labeled_tx =
            labeled_tx.clone();

        let labeler =
            labeler.clone();

        let handle =
            thread::spawn(move || {

            for batch
                in candidate_rx
            {
                let labeled =
                    labeler.label(batch);

                labeled_tx
                    .send(labeled)
                    .unwrap();
            }
        });

        handles.push(handle);
    }

    handles
}