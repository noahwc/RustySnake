use crate::node::*;
use crate::requests::*;
use std::collections::*;

pub struct Graph {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Node>,
    pub targets: Vec<Point>,
}

impl Graph {
    pub fn new(turn: &Turn) -> Graph {
        // row-major order
        let mut board = Vec::with_capacity(turn.board.width * turn.board.height);
        for row in 0..turn.board.height {
            for col in 0..turn.board.width {
                board.insert(turn.board.width * row + col, Node::new(Point {x: col, y: row}));
            }
        }
        
        Graph {
            width: turn.board.width,
            height: turn.board.height,
            board: board, 
            targets: Vec::new(),
        }
    }

    pub fn weight_nodes<F>(&mut self, heuristic: F) where F: Fn(Node) -> (i32, bool){
        for n in &mut self.board {
            let (weight, target) = heuristic(*n);
            n.weight = weight;
            if target {
                self.targets.push(n.point)
            }
        }
    }

    pub fn neighbours(&self, point: Point) -> Vec<Point> {
        let x = point.x;
        let y = point.y;
        let mut adj = Vec::new();
        
        if x > 0 {
            adj.push(Point {x: x-1, y: y});
        } if x < 10 {
            adj.push(Point {x: x+1, y: y});
        } if y > 0 {
            adj.push(Point {x: x, y: y-1})
        } if y < 10 {
            adj.push(Point {x: x, y: y+1})
        }

        adj
    }   

    pub fn djikstra(&mut self) {
        let w = self.width;
        let start = *self.board.iter().find(|&&node| node.has_head).unwrap();
        self.board[start.point.index(w)].cost = 0;
        let mut unvisited = HashSet::new();
        for node in &self.board {
            unvisited.insert(node.point);
        }
        unvisited.shrink_to_fit();

        while !unvisited.is_empty() {
            // get cheapest node
            let mut curr = *unvisited.iter().nth(0).unwrap();
            let mut curr_cost = self.board[curr.index(w)].cost;
            for point in &unvisited {
                let point_cost = self.board[point.index(w)].cost;  
                if point_cost < curr_cost {
                    curr = *point;
                    curr_cost = point_cost;
                }
            }
            // update neighbours cost and parent
            for adj in self.neighbours(curr) {
                let adj_node = self.board.get_mut(adj.index(w)).unwrap();
                if unvisited.contains(&adj) {
                    if curr_cost + adj_node.weight < adj_node.cost {
                        adj_node.cost = curr_cost + adj_node.weight;
                        adj_node.parent = Some(curr);
                    }
                }
            }
            unvisited.remove(&curr);
        }
    }
    
    pub fn path_to(&self, dest: &Point) -> Vec<Node> {
        let w = self.width;
        let mut path = vec![self.board[dest.index(w)]];
        loop {
            let curr = path.last().unwrap();
            match curr.parent {
                Some(point) => path.push(self.board[point.index(w)]),
                None => break
            }
        }
        path.reverse();
        path
    }

    pub fn connected_component(&self, source: Point) -> Vec<Node> {
        let mut cc = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::<Point>::with_capacity(self.height*self.width);
        queue.push_back(source);
        
        while !queue.is_empty() {
            let curr = self.board[queue.pop_front().unwrap().index(self.width)];
            match visited.get(&curr.point) {
                Some(_p) => continue,
                None => {
                    if !curr.has_snake || curr.has_tail{
                        cc.push(curr);
                        for point in self.neighbours(curr.point) {
                            queue.push_back(point)
                        }
                    }
                    visited.insert(curr.point);
                }
            }
        }

        return cc
    }

    pub fn is_safe(&self, path: &Vec<Node>, len: usize) -> bool {
        let source = path.last().unwrap().point;
        let cc = self.connected_component(source);

        if len < cc.len() {
            return true
        }
        
        for node in cc {
            if node.has_tail {
                return true
            }
        }

        return false
    }
}