#[derive(Debug, Clone)]
pub struct Evaluation {

    // centipawn
    pub cp: Option<i32>,

    // mate in N
    pub mate: Option<i32>,

    pub depth: u32,

    pub nodes: u64,

    pub pv: Vec<String>,
}