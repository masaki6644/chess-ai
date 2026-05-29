use shakmaty::{
    fen::Fen,
    CastlingMode,
    Chess,
    Position,
    uci::Uci,
};

pub fn play_move(

    fen: &str,

    mv: &str,
)
-> String
{
    let fen: Fen =
        fen.parse().unwrap();

    let mut pos: Chess =
        fen.into_position(
            CastlingMode::Standard,
        )
        .unwrap();

    let uci: Uci =
        mv.parse().unwrap();

    let mv =
        uci.to_move(
            &pos
        ).unwrap();

    pos.play_unchecked(
        &mv
    );

    Fen::from_position(
        pos,
        shakmaty::EnPassantMode::Legal,
    )
    .to_string()
}