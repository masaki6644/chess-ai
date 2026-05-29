pub struct SearchPolicy {

    pub mv: String,

    pub visits: u32,

    pub probability: f32,
}

pub struct SearchResult {

    // [-1, 1]
    pub value: f32,

    pub policy: Vec<SearchPolicy>,
}