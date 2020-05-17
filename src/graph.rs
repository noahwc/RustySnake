use crate::node::Node;
use crate::requests::Point;
use std::collections::HashMap;

pub struct Graph {
    pub board: [[Node; 11]; 11],
    pub height: usize,
    pub width: usize,
}

impl Graph {
    // constructor
    pub fn new() -> Graph {
        Graph {
            board: new_board(),
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
        if x > 1 {
            neighbours.push(self.board[x-1][y]);            
        }
        if y < 10 {
            neighbours.push(self.board[x][y+1]);
        }
        if y > 1 {
            neighbours.push(self.board[x][y-1]);
        }

        neighbours
    }   

    pub fn djikstra(&self, start: Node, dest: Node) -> Option<Vec<Node>> {
        
        let mut path = Vec::new();
        let mut map = HashMap::<Node, Vertex>::new();
        let max_cost = 128; 
        let start_cost = 1;      

        // initialize map with a Vertex for each node on board and fills visited
        for &n in self.board.iter().flat_map(|n| n.iter()) {
            if n == start {
                map.insert(n ,Vertex::new(n, start_cost));
            }
            else {
                map.insert(n ,Vertex::new(n, max_cost));
            }
        };

        // while not at destination
        while !map.get(&dest)?.visited {
            let mut curr_node = start;
            let mut curr_vert = Vertex::new(start, max_cost);
            // get (node, vertex) with lowest cost
            for (k, v) in &map {
                if !v.visited && v.cost < curr_vert.cost {
                    curr_node = *k;
                    curr_vert = *v;
                }
            }
            
            // update neighboor cost
            for nb in &self.get_neighbours(curr_node) {
                match map.get_mut(&nb) {
                    Some(v) => {
                        if !v.visited && v.cost > curr_vert.cost + nb.weight {
                            v.cost = curr_vert.cost + nb.weight;
                            v.parent = curr_node;
                        }
                    },
                    None => continue
                }
            }
            // mark current node as visited
            match map.get_mut(&curr_node) {
                Some(v) => v.visited = true,
                None => println!("{:?} NOT IN MAP", curr_node),
            }
        }

        // traceback path
        let mut n = dest;
        loop {
            match map.get(&n) {
                Some(v) => {
                    path.push(n);
                    if n!=v.parent{
                        n = v.parent;
                    } else {
                        break
                    }
                },
                None => break
            }
        }

        // return path encapsulated in Option
        if path.is_empty() {
            println!("NO PATH");
            return None
        } else {
            path.reverse();
            return Some(path)
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
    pub visited: bool
}

impl Vertex {
    pub fn new(parent: Node, cost: i32) -> Vertex {
        Vertex {
            cost,
            parent,
            visited: false,
        }
    }
}


