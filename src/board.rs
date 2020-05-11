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
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&mut Node){
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            heuristic(n);
        }
    }

    pub fn get_neighbours(&mut self, n: &Node) -> Vec<Neighbour> {
        let mut neighbours = Vec::new();
        let x = n.point.x;
        let y = n.point.y;

        if x < 10 {
            neighbours.push(Neighbour::Right(self.board[x+1][y]));
        }
        if x < 1 {
            neighbours.push(Neighbour::Left(self.board[x-1][y]));            
        }
        if y < 10 {
            neighbours.push(Neighbour::Up(self.board[x][y+1]));
        }
        if y < 1 {
            neighbours.push(Neighbour::Down(self.board[x][y-1]));
        }

        neighbours
    }

}

// pub fn dijkstra (&mut self){
//     let mut score_map = [[Default::default(); 11]; 11];
//     let mut visited = [[Default::default(); 11]; 11];
//     for j in 0..11 {
//         for i in 0..11 {
//             visited = 0
//             score_map[i][j] = u8::max_value();
//         }
//     }
// }


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

pub enum Neighbour {
    Left(Node),
    Right(Node),
    Up(Node),
    Down(Node),
}