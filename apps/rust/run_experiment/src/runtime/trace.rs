use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{
    Receiver,
    Sender,
};

use trace::bus::TraceBus;
use trace::event::TraceEvent;

pub fn spawn_trace_bus(

    rx:
        Receiver<TraceEvent>,

    ui_tx:
        Sender<TraceEvent>,

    ana_tx:
        Sender<TraceEvent>,
)
-> JoinHandle<()>
{
    thread::spawn(move || {

        let bus =
            TraceBus::new(
                rx,
                ui_tx,
                ana_tx,
            );

        bus.run();
    })
}