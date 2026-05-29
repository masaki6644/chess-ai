use shakmaty::{
    fen::Fen,
    CastlingMode,
    Chess,
    Position,
};

pub fn legal_moves(
    fen: &str,
) -> Vec<String> {

    // =========================
    // parse fen
    // =========================
    let fen: Fen =
        fen.parse().unwrap();

    let pos: Chess =
        fen.into_position(
            CastlingMode::Standard,
        )
        .unwrap();

    // =========================
    // legal moves
    // =========================
    pos.legal_moves()
        .into_iter()
        .map(|mv| {

            mv.to_uci(
                CastlingMode::Standard
            )
            .to_string()

        })
        .collect()
}