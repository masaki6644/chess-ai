use std::collections::HashMap;

use crate::event::TraceEvent;
use pipeline::filter::FilterReason;
use core::Game;

pub struct TraceCollector {
    // ===== game =====
    pub games_seen: usize,
    pub games_accepted: usize,
    pub games_filtered: usize,

    pub filtered_reasons: HashMap<FilterReason, usize>,

    // ===== positions =====
    pub expanded_total: usize,
    pub scored_total: usize,
    pub selected_total: usize,

    // ===== distribution =====
    pub expanded_hist: Vec<usize>,
    pub scored_hist: Vec<usize>,
    pub selected_hist: Vec<usize>,

    // ===== errors =====
    pub errors: usize,

        pub no_result_samples: Vec<Game>, 
}

impl TraceCollector {
    pub fn new() -> Self {
        Self {
            games_seen: 0,
            games_accepted: 0,
            games_filtered: 0,

            filtered_reasons: HashMap::new(),

            expanded_total: 0,
            scored_total: 0,
            selected_total: 0,

            expanded_hist: Vec::new(),
            scored_hist: Vec::new(),
            selected_hist: Vec::new(),

            errors: 0,
            no_result_samples:Vec::new(),
        }
    }

    pub fn record(&mut self, event: TraceEvent) {
        match event {
            // ===== game =====
            TraceEvent::GameSeen => {
                self.games_seen += 1;
            }
            TraceEvent::GameAccepted => {
                self.games_accepted += 1;
            }
            TraceEvent::GameFiltered { reason,game } => {
                self.games_filtered += 1;

                *self.filtered_reasons.entry(reason).or_insert(0) += 1;

                    // ★ NoResultだけサンプル保存（最大5件）
                if reason == FilterReason::NoResult && self.no_result_samples.len() < 5 {
                    self.no_result_samples.push(game.clone());
                }
            }

            // ===== pipeline =====
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

            // ===== error =====
            TraceEvent::Error { .. } => {
                self.errors += 1;
            }
        }
    }

    fn stats(hist: &[usize]) -> (f64, usize, usize) {
        if hist.is_empty() {
            return (0.0, 0, 0);
        }

        let sum: usize = hist.iter().sum();
        let avg = sum as f64 / hist.len() as f64;
        let min = *hist.iter().min().unwrap();
        let max = *hist.iter().max().unwrap();

        (avg, min, max)
    }

    pub fn print_summary(&self) {
        println!("\n===== FINAL SUMMARY =====");

        println!("games_seen      : {}", self.games_seen);
        println!("games_accepted  : {}", self.games_accepted);
        println!("games_filtered  : {}", self.games_filtered);

        println!("expanded_total  : {}", self.expanded_total);
        println!("scored_total    : {}", self.scored_total);
        println!("selected_total  : {}", self.selected_total);

        let (e_avg, e_min, e_max) = Self::stats(&self.expanded_hist);
        let (s_avg, s_min, s_max) = Self::stats(&self.scored_hist);
        let (sel_avg, sel_min, sel_max) = Self::stats(&self.selected_hist);

        println!("\n--- filter reasons ---");
        for (reason, count) in &self.filtered_reasons {
            println!("{:?}: {}", reason, count);
        }

        println!("\n--- expanded ---");
        println!("avg: {:.2}, min: {}, max: {}", e_avg, e_min, e_max);

        println!("\n--- scored ---");
        println!("avg: {:.2}, min: {}, max: {}", s_avg, s_min, s_max);

        println!("\n--- selected ---");
        println!("avg: {:.2}, min: {}, max: {}", sel_avg, sel_min, sel_max);

        if self.expanded_total > 0 {
            let ratio = self.selected_total as f64 / self.expanded_total as f64;
            println!("\nselection_ratio : {:.4}", ratio);
        }

        println!("\nerrors          : {}", self.errors);
        println!("========================\n");
        println!("\n--- NoResult samples ---");
        for (i, g) in self.no_result_samples.iter().enumerate() {
            println!("sample {}: moves={}: white_elo={:?}: black_elo={:?}", i, g.moves.len(),g.white_elo,g.black_elo);
        }
    }
}