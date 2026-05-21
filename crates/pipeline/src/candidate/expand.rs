use core::{Game, Position};
use shakmaty::Position as _;

use crate::candidate::types::PositionSample;

pub fn expand(game: &Game) -> Vec<PositionSample> {
    let mut pos = Position::default();
    let mut out = Vec::with_capacity(game.moves.len());

    let total = game.moves.len();

    for (i, mv) in game.moves.iter().enumerate() {

        // 現在の局面をサンプルとして保存
        out.push(PositionSample {
            pos: pos.clone(),
            ply: i,
            total_plies: total,
        });

        // 次の局面へ進める
        pos.play_unchecked(mv);
    }

    out
}