use std::borrow::Cow;
use std::io::BufRead;

use shakmaty::{Chess, Position};

use pgn_reader::{
    BufferedReader,
    RawHeader,
    SanPlus,
    Visitor,
};

use core::{Game, Move};

// =========================
// streaming parser
// =========================
pub fn parse_pgn<R, F>(
    reader: R,
    on_game: F,
)
where
    R: BufRead,
    F: FnMut(Game),
{
    let mut pgn =
        BufferedReader::new(reader);

    let mut collector = Collector {
        on_game,

        current_moves: Vec::new(),

        pos: Chess::default(),

        invalid: false,

        result: None,
        white_elo: None,
        black_elo: None,
    };

    pgn.read_all(&mut collector)
        .unwrap();
}

// =========================
// collector
// =========================
struct Collector<F>
where
    F: FnMut(Game),
{
    // callback
    on_game: F,

    // current game state
    current_moves: Vec<Move>,

    pos: Chess,

    invalid: bool,

    result: Option<i8>,

    white_elo: Option<u32>,
    black_elo: Option<u32>,
}

impl<F> Visitor for Collector<F>
where
    F: FnMut(Game),
{
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
    // headers
    // =========================
    fn header(
        &mut self,
        key: &[u8],
        value: RawHeader<'_>,
    ) {

        let v: Cow<str> =
            value.decode_utf8()
                .unwrap_or_else(
                    |_| "".into()
                );

        match key {

            b"Result" => {

                self.result =
                    match v.as_ref() {

                    "1-0" => Some(1),

                    "0-1" => Some(-1),

                    "1/2-1/2" => Some(0),

                    _ => None,
                };
            }

            b"WhiteElo" => {

                self.white_elo =
                    v.parse::<u32>().ok();
            }

            b"BlackElo" => {

                self.black_elo =
                    v.parse::<u32>().ok();
            }

            _ => {}
        }
    }

    // =========================
    // moves
    // =========================
    fn san(
        &mut self,
        san_plus: SanPlus,
    ) {

        if self.invalid {
            return;
        }

        match san_plus
            .san
            .to_move(&self.pos)
        {

            Ok(mv) => {

                self.pos
                    .play_unchecked(&mv);

                self.current_moves
                    .push(mv);
            }

            Err(_) => {

                self.invalid = true;
            }
        }
    }

    // =========================
    // game finished
    // =========================
    fn end_game(&mut self) {

        if self.invalid {
            return;
        }

        if self.current_moves.is_empty() {
            return;
        }

        // =========================
        // emit immediately
        // =========================
        (self.on_game)(Game {

            moves:
                self.current_moves.clone(),

            result:
                self.result,

            white_elo:
                self.white_elo,

            black_elo:
                self.black_elo,
        });
    }
}