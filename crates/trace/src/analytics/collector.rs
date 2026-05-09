use std::collections::HashMap;

use crate::event::TraceEvent;
use pipeline::filter::FilterReason;

pub struct TraceAnalytics {
    // =========================
    // counters
    // =========================
    pub games_seen: usize,
    pub games_accepted: usize,
    pub games_filtered: usize,

    pub expanded_total: usize,
    pub scored_total: usize,
    pub selected_total: usize,

    pub errors: usize,

    // =========================
    // distributions
    // =========================
    pub filtered_reasons: HashMap<FilterReason, usize>,

    pub expanded_hist: Vec<usize>,
    pub scored_hist: Vec<usize>,
    pub selected_hist: Vec<usize>,
}

impl TraceAnalytics {
    pub fn new() -> Self {
        Self {
            games_seen: 0,
            games_accepted: 0,
            games_filtered: 0,

            expanded_total: 0,
            scored_total: 0,
            selected_total: 0,

            errors: 0,

            filtered_reasons: HashMap::new(),

            expanded_hist: Vec::new(),
            scored_hist: Vec::new(),
            selected_hist: Vec::new(),
        }
    }

    /// streaming ingest
    pub fn ingest(&mut self, event: TraceEvent) {
        match event {
            TraceEvent::GameSeen => {
                self.games_seen += 1;
            }

            TraceEvent::GameAccepted => {
                self.games_accepted += 1;
            }

            TraceEvent::GameFiltered { reason, .. } => {
                self.games_filtered += 1;
                *self.filtered_reasons.entry(reason).or_insert(0) += 1;
            }

            TraceEvent::Expanded { count } => {
                self.expanded_total += count;
                self.expanded_hist.push(count);
            }

            TraceEvent::Scored { count } => {
                self.scored_total += count;
                self.scored_hist.push(count);
            }

            TraceEvent::Selected { count } => {
                self.selected_total += count;
                self.selected_hist.push(count);
            }

            TraceEvent::Error { .. } => {
                self.errors += 1;
            }
        }
    }
}