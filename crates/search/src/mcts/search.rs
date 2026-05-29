use engine::traits::Engine;

use crate::mcts::backup::backup;
use crate::mcts::expansion::expand;
use crate::mcts::policy::visits_to_policy;
use crate::mcts::rollout::rollout;
use crate::mcts::selection::select_leaf;
use crate::mcts::tree::MctsTree;

use crate::types::{
    SearchPolicy,
    SearchResult,
};

pub fn run_mcts<E>(

    engine: &mut E,

    root_fen: &str,

    simulations: usize,

    depth: u32,

    c_puct: f32,
)
-> SearchResult
where
    E: Engine,
{
    // =========================
    // root side
    // =========================
    let to_play =
        side_to_move(root_fen);

    // =========================
    // tree
    // =========================
    let mut tree =
        MctsTree::new(
            root_fen.to_string(),
            to_play,
        );

    // =========================
    // simulations
    // =========================
    for _ in 0..simulations {

        // =====================
        // selection
        // =====================
        let path =
            select_leaf(
                &tree,
                c_puct,
            );

        let leaf_id =
            *path
                .last()
                .unwrap();

        // =====================
        // expansion
        // =====================
        if !tree.nodes[leaf_id]
            .expanded
        {
            expand(
                &mut tree,
                leaf_id,
            );
        }

        // =====================
        // evaluation
        // =====================
        let value =
            if tree.nodes[leaf_id]
                .terminal
            {
                -1.0
            }
            else {

                rollout(

                    engine,

                    &tree.nodes[leaf_id]
                        .fen,

                    depth,
                )
            };

        // =====================
        // backup
        // =====================
        backup(
            &mut tree,
            &path,
            value,
        );
    }

    // =========================
    // root statistics
    // =========================
    let root =
        tree.root();

    let visits: Vec<(String, u32)> =
        root.children
            .iter()
            .map(|(mv, child_id)| {

                let child =
                    &tree.nodes[*child_id];

                (
                    mv.clone(),
                    child.visits,
                )
            })
            .collect();

    let policy:
        Vec<SearchPolicy> =
            visits_to_policy(
                &visits
            );

    SearchResult {

        value:
            root.q(),

        policy,
    }
}

// =============================
// fen side to move
// =============================
fn side_to_move(
    fen: &str,
)
-> i8
{
    let side =
        fen
            .split_whitespace()
            .nth(1)
            .unwrap_or("w");

    match side {

        "w" => 1,

        "b" => -1,

        _ => 1,
    }
}