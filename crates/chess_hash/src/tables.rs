use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct ZobristTables {
    pub piece_square: [[[u64; 64]; 6]; 2], // color, piece, square
    pub side_to_move: u64,
    pub castling: [u64; 16], // 4bit mask
    pub en_passant: [u64; 8], // file
}

impl ZobristTables {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(42); // 固定seed（重要）

        let mut piece_square = [[[0; 64]; 6]; 2];
        for c in 0..2 {
            for p in 0..6 {
                for sq in 0..64 {
                    piece_square[c][p][sq] = rng.gen();
                }
            }
        }

        let side_to_move = rng.gen();

        let mut castling = [0; 16];
        for i in 0..16 {
            castling[i] = rng.gen();
        }

        let mut en_passant = [0; 8];
        for i in 0..8 {
            en_passant[i] = rng.gen();
        }

        Self {
            piece_square,
            side_to_move,
            castling,
            en_passant,
        }
    }
}