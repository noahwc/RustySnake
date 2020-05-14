use crate::requests::Turn;
use crate::board::Board;
use crate::node::Node;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub board: Board,
    pub paths: Vec<Vec<Node>>,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            board: Board::new(),
            paths: Vec::new(),
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