use core::Game;

pub trait GameFilter {
    fn accept(&self, game: &Game) -> bool;
}

pub struct NoFilter;

impl GameFilter for NoFilter {
    fn accept(&self, _game: &Game) -> bool {
        true
    }
}

pub struct MinLengthFilter {
    pub min_moves: usize,
}

impl GameFilter for MinLengthFilter {
    fn accept(&self, game: &Game) -> bool {
        game.moves.len() >= self.min_moves
    }
}