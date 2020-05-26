use crate::node::Node;
use crate::requests::Point;
use std::collections::BinaryHeap;

pub struct Graph {
    pub board: [[Node; 11]; 11],
    pub targets: Vec<Point>
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
    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(&mut Node) -> i32 {
        for n in self.board.iter_mut().flat_map(|r| r.iter_mut()) {
            n.weight = heuristic(n);
            if n.target {
                self.targets.push(n.point);
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

        // initialize nodes
        for node in self.board.iter_mut().flatten() {
            if node.point == start {
                node.cost = 0;
            } else {
                node.cost = 1024;
                node.visited = false;
                node.parent = None;
            }
        }

        // push start into pq
        pq.push(self.board[start.x][start.y]);

        while !pq.is_empty(){
            // pop pq
            let curr = pq.pop().expect("pq empty");
             
            // update neighbours and push into pq
            for point in self.get_neighbours(curr) {
                let mut nb = &mut self.board[point.x][point.y];
                if !nb.visited {
                    let new_cost = curr.cost + nb.weight;
                    if new_cost < nb.cost {
                        nb.cost = new_cost;
                        nb.parent = Some(curr.point)
                    }
                    pq.push(*nb);
                }
            }
            // mark curr_node as visited
            self.board[curr.point.x][curr.point.y].visited = true;
        } 
    }
    
    pub fn path_to(&self, dest: Node) -> Option<Vec<Node>> {
        let mut path = Vec::<Node>::new();
        // traceback path
        let mut curr = dest;
        path.push(curr);
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
