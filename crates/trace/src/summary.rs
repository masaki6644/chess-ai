use crate::collector::TraceCollector;
use crate::stats::basic_stats;

pub struct SummaryPrinter;

impl SummaryPrinter {
    pub fn print(c: &TraceCollector) {
        println!("\n===== FINAL SUMMARY =====");

        println!("games_seen      : {}", c.games_seen);
        println!("games_accepted  : {}", c.games_accepted);
        println!("games_filtered  : {}", c.games_filtered);

        println!("expanded_total  : {}", c.expanded_total);
        println!("scored_total    : {}", c.scored_total);
        println!("selected_total  : {}", c.selected_total);

        Self::print_reasons(c);
        Self::print_flow(c);

        println!("\nerrors          : {}", c.errors);
        println!("========================\n");
    }

    fn print_reasons(c: &TraceCollector) {
        println!("\n--- filter reasons ---");
        for (reason, count) in &c.filtered_reasons {
            println!("{:?}: {}", reason, count);
        }
    }

    fn print_flow(c: &TraceCollector) {
        let (e_avg, e_min, e_max) = basic_stats(&c.expanded_hist);
        let (s_avg, s_min, s_max) = basic_stats(&c.scored_hist);
        let (sel_avg, sel_min, sel_max) = basic_stats(&c.selected_hist);

        println!("\n--- expanded ---");
        println!("avg: {:.2}, min: {}, max: {}", e_avg, e_min, e_max);

        println!("\n--- scored ---");
        println!("avg: {:.2}, min: {}, max: {}", s_avg, s_min, s_max);

        println!("\n--- selected ---");
        println!("avg: {:.2}, min: {}, max: {}", sel_avg, sel_min, sel_max);

        if c.expanded_total > 0 {
            let ratio = c.selected_total as f64 / c.expanded_total as f64;
            println!("\nselection_ratio : {:.4}", ratio);
        }
    }
}