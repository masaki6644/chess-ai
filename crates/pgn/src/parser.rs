use std::io::BufRead;

use shakmaty::{Chess, Position};
use pgn_reader::{BufferedReader, SanPlus, Visitor};

use core::{Game, Move};

pub fn parse_pgn<R: BufRead>(reader: R) -> Vec<Game> {
    let mut pgn = BufferedReader::new(reader);

    let mut collector = Collector {
        games: Vec::new(),
        current_moves: Vec::new(),
        pos: Chess::default(),
        invalid: false,
    };

    pgn.read_all(&mut collector).unwrap();

    collector.games
}

struct Collector {
    games: Vec<Game>,
    current_moves: Vec<Move>,
    pos: Chess,
    invalid: bool,
}

impl Visitor for Collector {
    type Result = ();

    fn begin_game(&mut self) {
        self.current_moves.clear();
        self.pos = Chess::default();
        self.invalid = false;
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.invalid {
            return;
        }

        match san_plus.san.to_move(&self.pos) {
            Ok(mv) => {
                self.pos.play_unchecked(&mv);
                self.current_moves.push(mv);
            }
            Err(_) => {
                self.invalid = true;
            }
        }
    }

    fn end_game(&mut self) {
        if !self.invalid && !self.current_moves.is_empty() {
            self.games.push(Game {
                moves: self.current_moves.clone(),
            });
        }
    }
}