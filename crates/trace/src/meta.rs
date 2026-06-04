use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]

pub struct GameMeta {
    pub game_id: u64,
    pub moves: usize,
    pub white_elo: Option<u32>,
    pub black_elo: Option<u32>,
}