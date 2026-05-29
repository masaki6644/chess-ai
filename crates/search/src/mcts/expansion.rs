use core::movegen::legal_moves;
use core::play::play_move;

use crate::mcts::node::MctsNode;
use crate::mcts::tree::MctsTree;

pub fn expand(

    tree: &mut MctsTree,

    node_id: usize,
)
{
    let fen =
        tree.nodes[node_id]
            .fen
            .clone();

    // =========================
    // legal moves
    // =========================
    let moves =
        legal_moves(&fen);

    tree.nodes[node_id]
        .legal_moves =
        moves.clone();

    // =========================
    // terminal
    // =========================
    if moves.is_empty() {

        tree.nodes[node_id]
            .terminal = true;

        tree.nodes[node_id]
            .expanded = true;

        return;
    }

    // =========================
    // uniform prior
    // =========================
    let prior =
        1.0 / moves.len() as f32;

    let next_to_play =
        -tree.nodes[node_id]
            .to_play;

    // =========================
    // child generation
    // =========================
    for mv in moves {

        let child_fen =
            play_move(
                &fen,
                &mv,
            );

        let child =
            MctsNode::new_child(

                child_fen,

                node_id,

                mv.clone(),

                prior,

                next_to_play,
            );

        let child_id =
            tree.add_node(child);

        tree.nodes[node_id]
            .children
            .insert(
                mv,
                child_id,
            );
    }

    tree.nodes[node_id]
        .expanded = true;
}