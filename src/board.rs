use crate::node::Node;
use crate::requests::{Point, Turn};
use std::collections::HashMap;

pub struct Board {
    pub board: [[Node; 11]; 11],
    height: usize,
    width: usize,
}

impl Board {
    // constructor
    pub fn new(t: &Turn) -> Board {
        Board {
            board: new_board(&t),
            height: 11,
            width: 11,
        }
    }
    // methods
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&mut Node){
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            heuristic(n);
        }
    }

    pub fn get_neighbours(&mut self, x: i8, y:i8){
        let mut neighbours = Vec::new();
        if(x - 1 >= 0){
            let mut neighbour = HashMap::new();
            neighbour.insert("x", x - 1);
            neighbour.insert("y", y);
        }
        if(x + 1 < 11){
            let mut neighbour = HashMap::new();
            neighbour.insert("x", x + 1);
            neighbour.insert("y", y);
        }
        if(y - 1 >= 0){
            let mut neighbour = HashMap::new();
            neighbour.insert("x", x);
            neighbour.insert("y", y - 1);
        }
        if(y + 1 < 11){
            let mut neighbour = HashMap::new();
            neighbour.insert("x", x);
            neighbour.insert("y", y + 1);
        }
        return neighbours;
    }
}


// Helper
pub fn new_board(t: &Turn) -> [[Node; 11]; 11] {
    let mut b = [[Default::default(); 11]; 11]; 
    for j in 0..11 {
        for i in 0..11 {
            b[i][j] = Node::new(&t, &Point{x: i, y: j}, 0);
        }
    }
    return b
}