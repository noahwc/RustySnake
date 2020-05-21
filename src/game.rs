use crate::requests::Turn;
use crate::graph::Graph;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub graph: Graph,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            graph: Graph::new(),
        }        
    }   
}