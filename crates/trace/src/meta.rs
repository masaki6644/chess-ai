#[derive(Debug, Clone)]
pub struct GameMeta {
    pub game_id: u64,
    pub moves: usize,
    pub white_elo: Option<u32>,
    pub black_elo: Option<u32>,
}