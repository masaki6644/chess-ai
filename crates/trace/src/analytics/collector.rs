use std::collections::HashMap;

use crate::event::TraceEvent;
use pipeline::candidate::filter::FilterReason;

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

    pub expanded_min: usize,
    pub expanded_max: usize,

    pub errors: usize,

    // =========================
    // distributions
    // =========================
    pub filtered_reasons:
        HashMap<FilterReason, usize>,

    pub total_plies_hist: [usize; 8],

    pub score_hist: [usize; 10],

    pub selected_score_hist: [usize; 10],

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

            expanded_min: usize::MAX,
            expanded_max: 0,

            errors: 0,

            filtered_reasons:
                HashMap::new(),

            total_plies_hist: [0; 8],

            score_hist: [0; 10],

            selected_score_hist: [0; 10],

        }
    }

    /// streaming ingest
    pub fn ingest(
        &mut self,
        event: TraceEvent,
    ) {

        match event {

            TraceEvent::GameSeen => {
                self.games_seen += 1;
            }

            TraceEvent::GameAccepted => {
                self.games_accepted += 1;
            }

            TraceEvent::GameFiltered {
                reason,
                ..
            } => {

                self.games_filtered += 1;

                *self
                    .filtered_reasons
                    .entry(reason)
                    .or_insert(0) += 1;
            }

            TraceEvent::Expanded {
                count,
                total_plies,
            } => {

                self.expanded_total += count;

                self.expanded_min =
                    self.expanded_min.min(count);

                self.expanded_max =
                    self.expanded_max.max(count);

                let bucket = match total_plies {
                    0..=19 => 0,
                    20..=39 => 1,
                    40..=59 => 2,
                    60..=79 => 3,
                    80..=99 => 4,
                    100..=119 => 5,
                    120..=149 => 6,
                    _ => 7,
                };

                self.total_plies_hist[bucket] += 1;
            }

            TraceEvent::Scored {
                count,
                scores,
            } => {

                self.scored_total += count;

                for score in scores {

                    let bucket =
                        (
                            score
                                .clamp(
                                    0.0,
                                    0.9999,
                                )
                                * 10.0
                        ) as usize;

                    self.score_hist[bucket] += 1;
                }
            }

            TraceEvent::Selected {
                count,
                scores,
            } => {

                self.selected_total += count;

                for score in scores {

                    let bucket =
                        (
                            score
                            .clamp(0.0, 0.9999)
                            * 10.0
                        ) as usize;

                    self.selected_score_hist[bucket] += 1;
                }
            }

            TraceEvent::Error { .. } => {
                self.errors += 1;
            }

            _ => {}
        }
    }
}