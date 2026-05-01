use core::{Game, Position};
use shakmaty::Position as _; // ← これが正解（traitだけ読み込む）

pub fn expand(game: &Game) -> Vec<Position> {
    let mut pos = Position::default();
    let mut out = Vec::with_capacity(game.moves.len());

    for mv in &game.moves {
        out.push(pos.clone());
        pos.play_unchecked(mv);
    }

    out
}