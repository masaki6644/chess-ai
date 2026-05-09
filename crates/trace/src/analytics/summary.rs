use crate::analytics::collector::TraceAnalytics;
use crate::analytics::stats::basic_stats;

pub struct SummaryPrinter;

impl SummaryPrinter {
    pub fn print(a: &TraceAnalytics) {
        println!("\n===== FINAL SUMMARY =====");

        println!("games_seen      : {}", a.games_seen);
        println!("games_accepted  : {}", a.games_accepted);
        println!("games_filtered  : {}", a.games_filtered);

        println!("expanded_total  : {}", a.expanded_total);
        println!("scored_total    : {}", a.scored_total);
        println!("selected_total  : {}", a.selected_total);

        Self::print_reasons(a);
        Self::print_flow(a);

        println!("\nerrors          : {}", a.errors);
        println!("========================\n");
    }

    fn print_reasons(a: &TraceAnalytics) {
        println!("\n--- filter reasons ---");
        for (reason, count) in &a.filtered_reasons {
            println!("{:?}: {}", reason, count);
        }
    }

    fn print_flow(a: &TraceAnalytics) {
        let (e_avg, e_min, e_max) = basic_stats(&a.expanded_hist);
        let (s_avg, s_min, s_max) = basic_stats(&a.scored_hist);
        let (sel_avg, sel_min, sel_max) = basic_stats(&a.selected_hist);

        println!("\n--- expanded ---");
        println!("avg: {:.2}, min: {}, max: {}", e_avg, e_min, e_max);

        println!("\n--- scored ---");
        println!("avg: {:.2}, min: {}, max: {}", s_avg, s_min, s_max);

        println!("\n--- selected ---");
        println!("avg: {:.2}, min: {}, max: {}", sel_avg, sel_min, sel_max);

        if a.expanded_total > 0 {
            let ratio = a.selected_total as f64 / a.expanded_total as f64;
            println!("\nselection_ratio : {:.4}", ratio);
        }
    }
}