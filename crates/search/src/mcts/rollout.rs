use engine::traits::Engine;

pub fn rollout<E>(

    engine: &mut E,

    fen: &str,

    depth: u32,
)
-> f32
where
    E: Engine,
{
    let eval =
        engine
            .evaluate(
                fen,
                depth,
            )
            .unwrap();

    // =========================
    // mate
    // =========================
    if let Some(mate) =
        eval.mate
    {
        return if mate > 0 {
            1.0
        } else {
            -1.0
        };
    }

    // =========================
    // centipawn
    // =========================
    let cp =
        eval.cp.unwrap_or(0);

    let cp =
        cp.clamp(-1000, 1000);

    cp as f32 / 1000.0
}