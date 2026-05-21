use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::Receiver;

use trace::analytics::collector::TraceAnalytics;
use trace::event::TraceEvent;

pub fn spawn_analytics(

    ana_rx:
        Receiver<TraceEvent>,
)
-> JoinHandle<TraceAnalytics>
{
    thread::spawn(move || {

        let mut analytics =
            TraceAnalytics::new();

        for event in ana_rx {

            analytics.ingest(event);
        }

        analytics
    })
}