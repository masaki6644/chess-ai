use crate::mcts::tree::MctsTree;

pub fn select_leaf(

    tree: &MctsTree,

    c_puct: f32,
)
-> Vec<usize>
{
    let mut path =
        vec![0usize];

    let mut current =
        0usize;

    loop {

        let node =
            &tree.nodes[current];

        // =========================
        // stop at leaf
        // =========================
        if !node.expanded
            || node.children.is_empty()
        {
            break;
        }

        let parent_visits =
            node.visits.max(1)
            as f32;

        let mut best_score =
            f32::NEG_INFINITY;

        let mut best_child =
            None;

        // =========================
        // PUCT
        // =========================
        for child_id
            in node.children.values()
        {
            let child =
                &tree.nodes[*child_id];

            let q =
                child.q();

            let u =
                c_puct
                * child.prior
                * parent_visits.sqrt()
                / (1.0
                    + child.visits as f32);

            let score =
                q + u;

            if score > best_score {

                best_score =
                    score;

                best_child =
                    Some(*child_id);
            }
        }

        let next =
            best_child
                .expect(
                    "selection failed"
                );

        current = next;

        path.push(current);
    }

    path
}