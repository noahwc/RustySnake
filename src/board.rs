use crate::node::Node;
use crate::requests::{Point, Turn};

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