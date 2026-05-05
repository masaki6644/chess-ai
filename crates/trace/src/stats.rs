pub fn basic_stats(hist: &[usize]) -> (f64, usize, usize) {
    if hist.is_empty() {
        return (0.0, 0, 0);
    }

    let sum: usize = hist.iter().sum();
    let avg = sum as f64 / hist.len() as f64;
    let min = *hist.iter().min().unwrap();
    let max = *hist.iter().max().unwrap();

    (avg, min, max)
}