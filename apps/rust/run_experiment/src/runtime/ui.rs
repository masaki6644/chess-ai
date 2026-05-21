use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::Receiver;

use trace::event::TraceEvent;

pub fn spawn_ui(

    ui_rx:
        Receiver<TraceEvent>,

    total_files: usize,

    workers: usize,
)
-> JoinHandle<()>
{
    thread::spawn(move || {

        trace::ui::ui_loop::run_ui_loop(
            ui_rx,
            total_files,
            workers,
        );
    })
}