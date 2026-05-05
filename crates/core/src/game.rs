use crate::Move;

#[derive(Clone)]
pub struct Game {
    pub moves: Vec<Move>,
        // ★追加
    pub result: Option<i8>,      // 1:白勝ち, 0:引き分け, -1:黒勝ち
    pub white_elo: Option<u32>,
    pub black_elo: Option<u32>,
}