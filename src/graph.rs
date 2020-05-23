use crate::node::Node;
use crate::requests::Point;
use std::collections::BinaryHeap;

pub struct Graph {
    pub board: [[Node; 11]; 11],
    pub targets: Vec<Node>
}

impl Graph {
    // constructor
    pub fn new() -> Graph {
        Graph {
            board: new_board(),
            targets: Vec::new()
        }
    }

    // methods
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&Node) -> i32{
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            n.weight = heuristic(n);
            if n.weight < 0 {
                self.targets.push(*n);
            }
        }
    }

    pub fn get_neighbours(&self, n: Node) -> Vec<Point> {
        let mut neighbours = Vec::new();
        let x = n.point.x;
        let y = n.point.y;

        if x < 10 {
            neighbours.push(Point{x: x+1, y: y});
        }
        if x > 0 {
            neighbours.push(Point{x: x-1, y: y});            
        }
        if y < 10 {
            neighbours.push(Point{x: x, y: y+1});
        }
        if y > 0 {
            neighbours.push(Point{x: x ,y: y-1});
        }

        neighbours
    }   

    pub fn djikstra(&mut self, start: Point){
        let mut pq = BinaryHeap::new();
        let mut start = self.board[start.x][start.y];

        // initialize nodes
        for node in self.board.iter_mut().flatten() {
            node.cost = 127;
            node.visited = false;
            node.parent = None;
        }
        start.cost = 0;

        // push start into pq
        pq.push(start);

        while !pq.is_empty(){
            // pop pq
            let mut curr = pq.pop().expect("pq empty");
             
            // update neighbours and push into pq
            for point in self.get_neighbours(curr) {
                let mut nb = self.board[point.x][point.y];
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
                Some(point) => {
                    curr = self.board[point.x][point.y];
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
