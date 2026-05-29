use std::collections::HashMap;

pub struct MctsNode {

    // =========================
    // position
    // =========================
    pub fen: String,

    // =========================
    // tree structure
    // =========================
    pub parent: Option<usize>,

    // move played from parent
    pub move_from_parent:
        Option<String>,

    // move -> child node id
    pub children:
        HashMap<String, usize>,

    // =========================
    // side to move
    // =========================
    //  1 = white
    // -1 = black
    pub to_play: i8,

    // =========================
    // search statistics
    // =========================
    pub visits: u32,

    pub value_sum: f32,

    // policy prior
    pub prior: f32,

    // cached mean Q
    pub q: f32,

    // =========================
    // expansion state
    // =========================
    pub expanded: bool,

    pub terminal: bool,

    // =========================
    // cached moves
    // =========================
    pub legal_moves:
        Vec<String>,
}

impl MctsNode {

    // =========================
    // root
    // =========================
    pub fn new_root(

        fen: String,

        to_play: i8,
    ) -> Self {

        Self {

            fen,

            parent: None,

            move_from_parent:
                None,

            children:
                HashMap::new(),

            to_play,

            visits: 0,

            value_sum: 0.0,

            prior: 1.0,

            q: 0.0,

            expanded: false,

            terminal: false,

            legal_moves:
                Vec::new(),
        }
    }

    // =========================
    // child
    // =========================
    pub fn new_child(

        fen: String,

        parent: usize,

        mv: String,

        prior: f32,

        to_play: i8,
    ) -> Self {

        Self {

            fen,

            parent:
                Some(parent),

            move_from_parent:
                Some(mv),

            children:
                HashMap::new(),

            to_play,

            visits: 0,

            value_sum: 0.0,

            prior,

            q: 0.0,

            expanded: false,

            terminal: false,

            legal_moves:
                Vec::new(),
        }
    }

    // =========================
    // mean value
    // =========================
    pub fn q(
        &self,
    ) -> f32 {

        self.q
    }

    // =========================
    // leaf
    // =========================
    pub fn is_leaf(
        &self,
    ) -> bool {

        !self.expanded
    }

    // =========================
    // backup update
    // =========================
    pub fn update(
        &mut self,
        value: f32,
    ) {

        self.visits += 1;

        self.value_sum += value;

        self.q =
            self.value_sum
            / self.visits as f32;
    }
}