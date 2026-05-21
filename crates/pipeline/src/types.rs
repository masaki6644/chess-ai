use shakmaty::Color;

// =========================
// CandidatePosition
// =========================
#[derive(Debug, Clone)]
pub struct CandidatePosition<F> {

    pub game_id: u64,

    pub ply: u16,

    pub fen: String,

    pub stm: Color,

    pub features: F,

    pub score: f32,
}

// =========================
// CandidateBatch
// =========================
#[derive(Debug, Clone)]
pub struct CandidateBatch<F> {

    pub positions:
        Vec<CandidatePosition<F>>,
}

// =========================
// PolicyTarget
// =========================
#[derive(Debug, Clone)]
pub struct PolicyTarget {

    pub mv: String,

    pub prob: f32,
}

// =========================
// ValueKind
// =========================
#[derive(Debug, Clone)]
pub enum ValueKind {

    Centipawn,

    WinRate,

    QValue,
}

// =========================
// LabeledPosition
// =========================
#[derive(Debug, Clone)]
pub struct LabeledPosition<F> {

    pub candidate:
        CandidatePosition<F>,

    pub policy:
        Vec<PolicyTarget>,

    pub value: f32,

    pub value_kind:
        ValueKind,
}

// =========================
// LabeledBatch
// =========================
#[derive(Debug, Clone)]
pub struct LabeledBatch<F> {

    pub positions:
        Vec<LabeledPosition<F>>,
}