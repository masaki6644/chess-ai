use crate::mcts::tree::MctsTree;

pub fn backup(

    tree: &mut MctsTree,

    path: &[usize],

    mut value: f32,
)
{
    for node_id
        in path.iter().rev()
    {
        let node =
            &mut tree.nodes[*node_id];

        node.update(value);

        // =========================
        // alternate perspective
        // =========================
        value = -value;
    }
}