use core::Game;

use pipeline::filter::FilterReason;

pub enum TraceEvent {
    GameSeen,
    GameAccepted,
    
    GameFiltered { reason: FilterReason,game:Game },

    Expanded { count: usize },
    Scored { count: usize },
    Selected { count: usize },

    Error { stage: &'static str },
}