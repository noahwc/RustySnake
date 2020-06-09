use super::node::*;
use super::requests::*;
use std::collections::*;
use std::cmp::Ordering;


pub struct Graph {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Node>,
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
        }
    }

    pub fn index(&self, point: &Point) -> usize {
        self.width * point.y + point.x
    }

    pub fn weight_nodes<F>(&mut self, heuristic: F) -> Vec<Node> where F: Fn(Node) -> (i32, bool){
        let mut targets = Vec::new();
        for n in &mut self.board {
            let (weight, target) = heuristic(*n);
            n.weight = weight;
            if target {
                targets.push(*n)
            }
        }
        targets
    }

    pub fn neighbours(&self, source: &Node) -> Vec<Node> {
        let x = source.point.x;
        let y = source.point.y;
        let mut adj = Vec::new();
        
        if x > 0 {
            adj.push( self.board[ self.index( &Point { x: x - 1, y: y } ) ] )
        } if x < 10 {
            adj.push( self.board[ self.index( &Point { x: x + 1, y: y } ) ] )
        } if y > 0 {
            adj.push( self.board[ self.index( &Point { x: x, y: y - 1 } ) ] )
        } if y < 10 {
            adj.push( self.board[ self.index( &Point { x: x, y: y + 1 } ) ] )
        }

        adj
    }   

    pub fn djikstra(&self, source: &Node, targets: &Vec<Node>) -> Vec<Vec<Node>> {
        let mut vertices = Vec::new();
        for n in &self.board {
            vertices.push(
                Vertex {
                    node: n,
                    cost: 9999,
                    parent: None
                }
            )
        }
        vertices[self.index(&source.point)].cost = 0;
        
        let mut pq = BinaryHeap::new();
        pq.push(&vertices[self.index(&source.point)]);

        while !pq.is_empty() {
            let curr_vertex = pq.pop().unwrap();

            for node in self.neighbours(&curr_vertex.node) {
                let nb_vertex = &mut vertices[self.index(&node.point)];
        
                if nb_vertex.cost > curr_vertex.cost + curr_vertex.node.weight {
                    nb_vertex.cost = curr_vertex.cost + curr_vertex.node.weight;
                    pq.push(&vertices[self.index(&node.point)]);
                }

            }

        }
        
        let paths = Vec::new();
        for &target in targets {
            let path = vec![target];
            loop {
                let curr_vertex = &vertices[self.index(&path.last().unwrap().point)];
                match curr_vertex.parent {
                    Some(node) => path.push(*curr_vertex.node),
                    None => break
                }
            }
            path.reverse();
            paths.push(path);
        }

        paths
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
        let result;

        if len < cc.len() {
            result = true;
        } 
        else if cc.iter().any(|node| node.has_tail){
            result = true;
        }
        else {
            result = false;
        }

        result
    }
}

pub struct Vertex<'a>{
    node: &'a Node,
    cost: i32,
    parent: Option<Node>,
}

impl<'a> Ord for Vertex<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
}

impl<'a> PartialOrd for Vertex<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Vertex<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<'a> Eq for Vertex<'a> {}