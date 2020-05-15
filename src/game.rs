use crate::requests::Turn;
use crate::graph::Graph;
use crate::node::Node;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub graph: Graph,
    pub paths: Vec<Vec<Node>>,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            graph: Graph::new(),
            paths: Vec::new(),
        }        
    }
    
    // Methods
    pub fn best_path(&mut self) -> &Vec<Node> {
        self.paths.sort_by(|a, b| cost(&b).cmp(&cost(&a)));
        &self.paths[0]
    }

    pub fn our_head(&self) -> &Node {
        self.graph.board
        .iter()
        .flat_map(|r| r.into_iter())
        .find(|&n| n.point == self.turn.you.body[0])
        .unwrap()
    }
}

// Helpers
pub fn cost(v: &Vec<Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}