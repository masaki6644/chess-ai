use std::io::BufRead;
use std::borrow::Cow;

use shakmaty::{Chess, Position};
use pgn_reader::{BufferedReader, SanPlus, Visitor, RawHeader};

use core::{Game, Move};

pub fn parse_pgn<R: BufRead>(reader: R) -> Vec<Game> {
    let mut pgn = BufferedReader::new(reader);

    let mut collector = Collector {
        games: Vec::new(),
        current_moves: Vec::new(),
        pos: Chess::default(),
        invalid: false,

        result: None,
        white_elo: None,
        black_elo: None,
    };

    pgn.read_all(&mut collector).unwrap();

    collector.games
}

struct Collector {
    games: Vec<Game>,
    current_moves: Vec<Move>,
    pos: Chess,
    invalid: bool,

    result: Option<i8>,
    white_elo: Option<i32>,
    black_elo: Option<i32>,
}

impl Visitor for Collector {
    type Result = ();

    fn begin_game(&mut self) {
        self.current_moves.clear();
        self.pos = Chess::default();
        self.invalid = false;

        self.result = None;
        self.white_elo = None;
        self.black_elo = None;
    }

    // =========================
    // ✔ 修正ポイント
    // =========================
    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        let v: Cow<str> = value.decode_utf8().unwrap_or_else(|_| "".into());

        match key {
            b"Result" => {
                self.result = match v.as_ref() {
                    "1-0" => Some(1),
                    "0-1" => Some(-1),
                    "1/2-1/2" => Some(0),
                    _ => None,
                };
            }
            b"WhiteElo" => {
                self.white_elo = v.parse::<i32>().ok();
            }
            b"BlackElo" => {
                self.black_elo = v.parse::<i32>().ok();
            }
            _ => {}
        }
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
                result: self.result,
                white_elo: self.white_elo,
                black_elo: self.black_elo,
            });
        }
    }
}