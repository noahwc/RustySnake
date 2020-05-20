use crate::requests::{Turn, Point};
use crate::graph::Graph;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub graph: Graph,
    pub our_head: Point,
    pub our_tail: Point,
    pub snake_weight: i32,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            graph: Graph::new(),
            our_head: *t.you.body.first().unwrap(),
            our_tail: *t.you.body.last().unwrap(),
            snake_weight: 122,
        }        
    }   
}