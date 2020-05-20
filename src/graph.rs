use crate::node::Node;
use crate::requests::Point;
use std::collections::HashMap;
use std::collections::BinaryHeap;

pub struct Graph {
    pub board: [[Node; 11]; 11],
    pub map: HashMap::<Node, Vertex>,
    pub height: usize,
    pub width: usize,
}

impl Graph {
    // constructor
    pub fn new() -> Graph {
        Graph {
            board: new_board(),
            map: HashMap::<Node, Vertex>::new(),
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

    pub fn djikstra2(&mut self, start: Node){
        let mut pq = BinaryHeap::new();

        // TODO: write pq sorting

        let mut unvisited = Vec::new();
        let max_cost = 128; 
        let start_cost = 1;      

        // initialize map with a Vertex for each node on board and fills visited
        for &n in self.board.iter().flat_map(|n| n.iter()) {
            if n == start {
                self.map.insert(n ,Vertex::new(n, start_cost));
            }
            else {
                self.map.insert(n ,Vertex::new(n, max_cost));
            }
            unvisited.push(n);
        };

        pq.push(start)

        while !pq.is_empty(){
             // take node with lowest cost
             let curr_node = pq.pop();
             let curr_vert = *self.map.get(&curr_node).unwrap();
             
             // update neighboor cost
             for nb in &self.get_neighbours(curr_node) {
                 if unvisited.contains(nb) {
                     match self.map.get_mut(&nb) {
                         Some(v) => {
                             if v.cost > curr_vert.cost + nb.weight {
                                    v.cost = curr_vert.cost + nb.weight;
                                    v.parent = curr_node;
                                    // TODO: push nodes to pq
                             }
                         },
                         None => continue
                     }
                 }
             }
        } 
    }

    pub fn djikstra(&mut self, start: Node) {
        let mut unvisited = Vec::new();
        let max_cost = 128; 
        let start_cost = 1;      

        // initialize map with a Vertex for each node on board and fills visited
        for &n in self.board.iter().flat_map(|n| n.iter()) {
            if n == start {
                self.map.insert(n ,Vertex::new(n, start_cost));
            }
            else {
                self.map.insert(n ,Vertex::new(n, max_cost));
            }
            unvisited.push(n);
        };

        // while there are unvisited nodes
        while !unvisited.is_empty() {
            // sort unvisited nodes by cost
            unvisited.sort_by(|a, b| {
                self.map.get(a).unwrap().cost
                .cmp(&self.map.get(b).unwrap().cost)
            });

            // take node with lowest cost
            let curr_node = unvisited.remove(0);
            let curr_vert = *self.map.get(&curr_node).unwrap();
            
            // update neighboor cost
            for nb in &self.get_neighbours(curr_node) {
                if unvisited.contains(nb) {
                    match self.map.get_mut(&nb) {
                        Some(v) => {
                            if v.cost > curr_vert.cost + nb.weight {
                                v.cost = curr_vert.cost + nb.weight;
                                v.parent = curr_node;
                            }
                        },
                        None => continue
                    }
                }
            }
        }
    }
    
    pub fn path_to(&self, dest: Node) -> Option<Vec<Node>> {
        let mut path = Vec::<Node>::new();
        // traceback path
        let mut n = dest;
        loop {
            let v = self.map.get(&n).unwrap();
            path.push(n);
            if n != v.parent {
                n = v.parent;
            } else {
                break
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

    pub fn get_node (&self, &p: &Point) -> Option<Node> {
        for &n in self.board.iter().flat_map(|r| r.into_iter()) {
            if n.point == p {
                return Some(n)
            }
        }
        return None
    }
}


// Helpers
pub fn new_board() -> [[Node; 11]; 11] {
    let mut b = [[Default::default(); 11]; 11]; 
    for j in 0..11 {
        for i in 0..11 {
            b[i][j] = Node::new(&Point{x: i, y: j}, 0);
        }
    }
    return b
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub cost: i32,
    pub parent: Node,
}

impl Vertex {
    pub fn new(parent: Node, cost: i32) -> Vertex {
        Vertex {
            cost,
            parent,
        }
    }
}


