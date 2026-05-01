use crate::event::TraceEvent;

pub struct TraceCollector {
    // ===== game =====
    pub games_seen: usize,
    pub games_accepted: usize,
    pub games_filtered: usize,

    // ===== positions =====
    pub expanded_total: usize,
    pub scored_total: usize,
    pub selected_total: usize,

    // ===== errors =====
    pub errors: usize,
}

impl TraceCollector {
    pub fn new() -> Self {
        Self {
            games_seen: 0,
            games_accepted: 0,
            games_filtered: 0,
            expanded_total: 0,
            scored_total: 0,
            selected_total: 0,
            errors: 0,
        }
    }

    pub fn record(&mut self, event: TraceEvent) {
        match event {
            TraceEvent::GameSeen => {
                self.games_seen += 1;
            }
            TraceEvent::GameAccepted => {
                self.games_accepted += 1;
            }
            TraceEvent::GameFiltered => {
                self.games_filtered += 1;
            }

            TraceEvent::Expanded { positions } => {
                self.expanded_total += positions;
            }
            TraceEvent::Scored { positions } => {
                self.scored_total += positions;
            }
            TraceEvent::Selected { positions } => {
                self.selected_total += positions;
            }

            TraceEvent::Error { .. } => {
                self.errors += 1;
            }
        }
    }

    pub fn print_summary(&self) {
        println!("\n===== FINAL SUMMARY =====");

        println!("games_seen      : {}", self.games_seen);
        println!("games_accepted  : {}", self.games_accepted);
        println!("games_filtered  : {}", self.games_filtered);

        println!("expanded_total  : {}", self.expanded_total);
        println!("scored_total    : {}", self.scored_total);
        println!("selected_total  : {}", self.selected_total);

        // ===== 割合（重要）=====
        if self.expanded_total > 0 {
            let ratio = self.selected_total as f64 / self.expanded_total as f64;
            println!("selection_ratio : {:.4}", ratio);
        }

        println!("errors          : {}", self.errors);

        println!("========================\n");
    }
}