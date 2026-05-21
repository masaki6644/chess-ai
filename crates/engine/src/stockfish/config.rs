#[derive(Clone)]
pub struct StockfishConfig {

    pub path: String,

    pub threads: usize,

    pub hash_mb: usize,
}