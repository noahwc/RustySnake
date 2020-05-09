extern crate ndarray;

use ndarray::prelude::*;
use crate::requests::{Turn, Point};
use crate::node::Node;

#[derive(Debug)]
pub struct Game<'a> {
    pub turn: &'a Turn,
    pub grid: Array2::<Node>,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            grid: new_array(&t),
        }        
    }
}

fn new_array (t: &Turn) -> Array2::<Node> {
    let mut a = Array2::<Node>::default((t.board.width, t.board.height));

    for j in 0..t.board.height {
        for i in 0..t.board.width {
            a[[i,j]] = Node::new(&t, &Point{x: i as i32, y: j as i32}, 0);
        }
    }
    
    a
}