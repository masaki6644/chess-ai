use shakmaty::{
    Chess, Color, Piece, Role, Square, Position,
    EnPassantMode, CastlingSide,
};

use crate::tables::ZobristTables;
use crate::Hash;

pub struct Zobrist {
    pub tables: ZobristTables,
}

impl Zobrist {
    pub fn new() -> Self {
        Self {
            tables: ZobristTables::new(),
        }
    }

    // =========================
    // フルハッシュ
    // =========================
    pub fn hash(&self, pos: &Chess) -> Hash {
        let mut h = 0;

        for (sq, piece) in pos.board().clone() {
            h ^= self.hash_piece(piece, sq);
        }

        if pos.turn() == Color::Black {
            h ^= self.tables.side_to_move;
        }

        let c = self.castling_index(pos);
        h ^= self.tables.castling[c];

        if let Some(ep) = pos.ep_square(EnPassantMode::Legal) {
            h ^= self.tables.en_passant[ep.file() as usize];
        }

        h
    }

    // =========================
    // incremental
    // =========================
    pub fn update(
        &self,
        prev: Hash,
        old: &Chess,
        new: &Chess,
    ) -> Hash {
        let mut h = prev;

        let old_board = old.board();
        let new_board = new.board();

        // remove
        for (sq, piece) in old_board.clone() {
            if new_board.piece_at(sq).is_none() {
                h ^= self.hash_piece(piece, sq);
            }
        }

        // add
        for (sq, piece) in new_board.clone() {
            if old_board.piece_at(sq).is_none() {
                h ^= self.hash_piece(piece, sq);
            }
        }

        // side to move
        if old.turn() != new.turn() {
            h ^= self.tables.side_to_move;
        }

        // castling
        let old_c = self.castling_index(old);
        let new_c = self.castling_index(new);

        if old_c != new_c {
            h ^= self.tables.castling[old_c];
            h ^= self.tables.castling[new_c];
        }

        // en passant
        let old_ep = old
            .ep_square(EnPassantMode::Legal)
            .map(|s| s.file() as usize);

        let new_ep = new
            .ep_square(EnPassantMode::Legal)
            .map(|s| s.file() as usize);

        if old_ep != new_ep {
            if let Some(f) = old_ep {
                h ^= self.tables.en_passant[f];
            }
            if let Some(f) = new_ep {
                h ^= self.tables.en_passant[f];
            }
        }

        h
    }

    // =========================
    // helper
    // =========================
    fn hash_piece(&self, piece: Piece, sq: Square) -> Hash {
        let c = match piece.color {
            Color::White => 0,
            Color::Black => 1,
        };

        let p = match piece.role {
            Role::Pawn => 0,
            Role::Knight => 1,
            Role::Bishop => 2,
            Role::Rook => 3,
            Role::Queen => 4,
            Role::King => 5,
        };

        self.tables.piece_square[c][p][sq as usize]
    }

    fn castling_index(&self, pos: &Chess) -> usize {
        let c = pos.castles();

        let mut rights = 0;

        // shakmaty 0.26: CastlingSideベース
        if c.has(Color::White, CastlingSide::KingSide) {
            rights |= 1;
        }
        if c.has(Color::White, CastlingSide::QueenSide) {
            rights |= 2;
        }
        if c.has(Color::Black, CastlingSide::KingSide) {
            rights |= 4;
        }
        if c.has(Color::Black, CastlingSide::QueenSide) {
            rights |= 8;
        }

        rights
    }
}