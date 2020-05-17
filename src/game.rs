use crate::requests::Turn;
use crate::graph::Graph;
use crate::node::Node;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub graph: Graph,
    pub paths: Vec<Vec<Node>>,
    pub our_head: Node,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            graph: Graph::new(),
            paths: Vec::new(),
            our_head: Node::new(&t.you.body[0], 0),
        }        
    }
    
    // Methods
    pub fn best_path(&mut self) -> &Vec<Node> {
        self.paths.sort_by(|a, b| cost(&b).cmp(&cost(&a)));
        &self.paths[0]
    }
}

// Helpers
pub fn cost(v: &Vec<Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}