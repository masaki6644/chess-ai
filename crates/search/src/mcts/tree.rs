use crate::mcts::node::MctsNode;

pub struct MctsTree {

    pub nodes:
        Vec<MctsNode>,
}

impl MctsTree {

    pub fn new(
        root_fen: String,
        to_play: i8,
    ) -> Self {

        Self {

            nodes: vec![
                MctsNode::new_root(
                    root_fen,
                    to_play,
                )
            ],
        }
    }

    pub fn root(
        &self,
    ) -> &MctsNode {

        &self.nodes[0]
    }

    pub fn root_mut(
        &mut self,
    ) -> &mut MctsNode {

        &mut self.nodes[0]
    }

    pub fn add_node(
        &mut self,
        node: MctsNode,
    ) -> usize {

        let id =
            self.nodes.len();

        self.nodes.push(node);

        id
    }
}