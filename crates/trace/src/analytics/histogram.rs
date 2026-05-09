pub struct Histogram {
    bins: Vec<usize>,
    bin_size: usize,
}

impl Histogram {
    pub fn new(bin_size: usize, max: usize) -> Self {
        let bins = vec![0; max / bin_size + 1];
        Self { bins, bin_size }
    }

    pub fn observe(&mut self, value: usize) {
        let idx = value / self.bin_size;
        if let Some(bin) = self.bins.get_mut(idx) {
            *bin += 1;
        }
    }

    pub fn bins(&self) -> &[usize] {
        &self.bins
    }
}