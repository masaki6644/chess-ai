use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{
    Receiver,
    Sender,
};

use experiment::config::ExperimentConfig;
use experiment::runner::run;

use pipeline::types::CandidateBatch;

use trace::event::TraceEvent;

pub fn spawn_parse_workers<F>(

    num_workers: usize,

    job_rx:
        Receiver<(usize, String)>,

    config:
        ExperimentConfig<F>,

    trace_tx:
        Sender<TraceEvent>,

    candidate_tx:
        Sender<CandidateBatch<F>>,
)
-> Vec<JoinHandle<()>>
where
    F:
        Clone
        + Send
        + Sync
        + 'static,
{
    let mut handles =
        Vec::new();

    for worker_id
        in 0..num_workers
    {
        let job_rx =
            job_rx.clone();

        let trace_tx =
            trace_tx.clone();

        let candidate_tx =
            candidate_tx.clone();

        let config =
            config.clone();

        let handle =
            thread::spawn(move || {

            for (
                file_id,
                path,
            ) in job_rx
            {
                let file =
                    File::open(&path)
                        .unwrap();

                let reader =
                    BufReader::new(file);

                run(
                    reader,

                    config.clone(),

                    trace_tx.clone(),

                    candidate_tx.clone(),

                    file_id as u64,

                    worker_id,

                    path.clone(),
                );
            }
        });

        handles.push(handle);
    }

    handles
}