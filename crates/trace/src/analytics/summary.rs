use crate::analytics::collector::TraceAnalytics;

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
        Self::print_expand(a);
        Self::print_scores(a);
        Self::print_selected_scores(a);
        Self::print_flow(a);

        println!("\nerrors          : {}", a.errors);

        println!("========================\n");
    }

    fn print_reasons(a: &TraceAnalytics) {

        println!("\n--- filter reasons ---");

        for (reason, count)
            in &a.filtered_reasons
        {
            println!("{:?}: {}", reason, count);
        }
    }

    fn print_expand(a: &TraceAnalytics) {

        println!("\n--- expanded ---");

        let avg =
            if a.games_accepted > 0 {

                a.expanded_total as f64
                    / a.games_accepted as f64

            } else {
                0.0
            };

        println!(
            "avg: {:.2}, min: {}, max: {}",
            avg,
            a.expanded_min,
            a.expanded_max,
        );

        println!("\n--- total plies ---");

        let labels = [
            "0-19",
            "20-39",
            "40-59",
            "60-79",
            "80-99",
            "100-119",
            "120-149",
            "150+",
        ];

        for (label, count)
            in labels
                .iter()
                .zip(a.total_plies_hist.iter())
        {
            println!("{:<10}: {}", label, count);
        }
    }

    fn print_scores(a: &TraceAnalytics) {

        println!("\n--- score distribution ---");

        let labels = [
            "0.0-0.1",
            "0.1-0.2",
            "0.2-0.3",
            "0.3-0.4",
            "0.4-0.5",
            "0.5-0.6",
            "0.6-0.7",
            "0.7-0.8",
            "0.8-0.9",
            "0.9-1.0",
        ];

        for ((label, scored), selected)
            in labels
                .iter()
                .zip(a.score_hist.iter())
                .zip(a.selected_score_hist.iter())
        {
            let ratio =
                if *scored > 0 {

                    *selected as f64
                        / *scored as f64

                } else {
                    0.0
                };

            println!(
                "{:<10}: scored={:<10} selected={:<10} ratio={:.4}",
                label,
                scored,
                selected,
                ratio,
            );
        }
    }

    fn print_selected_scores(
        a: &TraceAnalytics,
    ) {

        println!(
            "\n--- selected score distribution ---"
        );

        let labels = [
            "0.0-0.1",
            "0.1-0.2",
            "0.2-0.3",
            "0.3-0.4",
            "0.4-0.5",
            "0.5-0.6",
            "0.6-0.7",
            "0.7-0.8",
            "0.8-0.9",
            "0.9-1.0",
        ];

        for (label, count)
            in labels
                .iter()
                .zip(
                    a.selected_score_hist.iter()
                )
        {
            println!("{:<10}: {}", label, count);
        }
    }

    fn print_flow(a: &TraceAnalytics) {

        println!("\n--- selected ---");


        if a.expanded_total > 0 {

            let ratio =
                a.selected_total as f64
                    / a.expanded_total as f64;

            println!(
                "\nselection_ratio : {:.4}",
                ratio
            );
        }
    }
}