use crate::node::Node;
use crate::requests::Point;
use std::collections::BinaryHeap;

pub struct Graph {
    pub board: [[Node; 11]; 11],
}

impl Graph {
    // constructor
    pub fn new() -> Graph {
        Graph {
            board: new_board(),
        }
    }

    // methods
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&mut Node){
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            heuristic(n);
        }
    }

    pub fn get_neighbours(&self, n: Node) -> Vec<Node> {
        let mut neighbours = Vec::new();
        let x = n.point.x;
        let y = n.point.y;

        if x < 10 {
            neighbours.push(self.board[x+1][y]);
        }
        if x > 0 {
            neighbours.push(self.board[x-1][y]);            
        }
        if y < 10 {
            neighbours.push(self.board[x][y+1]);
        }
        if y > 0 {
            neighbours.push(self.board[x][y-1]);
        }

        neighbours
    }   

    pub fn djikstra(&mut self, start: Node){
        let mut pq = BinaryHeap::new();

        // initialize nodes
        for node in self.board.iter_mut().flatten() {
            node.cost = 128;
            node.visited = false;
            node.parent = None;
        }
        start.cost = 0;

        // push start into pq
        pq.push(start);

        while !pq.is_empty(){
            // pop pq
            let curr = pq.pop().expect("pq empty");
             
            // update neighbours and push into pq
            for nb in self.get_neighbours(curr) {
                if !nb.visited {
                    let new_cost = curr.cost + nb.weight;
                    if new_cost < nb.cost {
                        nb.cost = new_cost;
                        nb.parent = Some(curr.point)
                    }
                    pq.push(nb);
                }
            }
            // mark curr_node as visited
            curr.visited = true;
        } 
    }
    
    pub fn path_to(&self, dest: Node) -> Option<Vec<Node>> {
        let mut path = Vec::<Node>::new();
        // traceback path
        let mut curr = dest;
        loop {
            match curr.parent {
                Some(parent) => {
                    curr = self.get_node(parent).expect("no node!");
                    path.push(curr);
                },
                None => break
            }
        }
        // return path encapsulated in Option
        if path.len() > 1 {
            path.reverse();
            return Some(path)
        } else {
            return None
        }
    }

    pub fn get_node (&self, p: Point) -> Option<Node> {
        if p.x < 11 && p.y < 11 {
            Some(self.board[p.x][p.y])
        } else {
            None
        }

    }
}


// Helpers
pub fn new_board() -> [[Node; 11]; 11] {
    let mut b = [[Default::default(); 11]; 11]; 
    for j in 0..11 {
        for i in 0..11 {
            b[i][j] = Node::new(Point{x: i, y: j}, 0);
        }
    }
    return b
}
