use super::node::*;
use super::requests::*;
use std::collections::*;

pub struct Graph {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Node>,
}

impl Graph {
    pub fn new(turn: &Turn) -> Graph {
        // row-major order
        let mut board = Vec::with_capacity(turn.board.width * turn.board.height);
        for row in 0 .. turn.board.height {
            for col in 0 .. turn.board.width {
                board.insert(
                    turn.board.width * row + col,
                    Node::new(Point { x: col, y: row }),
                );
            }
        }

        Graph {
            width: turn.board.width,
            height: turn.board.height,
            board,
        }
    }

    pub fn weight_nodes<F>(&mut self, heuristic: F) -> Vec<Node>
    where F: Fn(Node) -> (usize, bool) {
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

    pub fn neighbours(&self, source: Point) -> Vec<Point> {
        let x = source.x;
        let y = source.y;
        let mut adj = Vec::new();

        if x > 0 {
            adj.push(Point { x: x - 1, y })
        }
        if x < self.width - 1 {
            adj.push(Point { x: x + 1, y })
        }
        if y > 0 {
            adj.push(Point { x, y: y - 1 })
        }
        if y < self.height - 1 {
            adj.push(Point { x, y: y + 1 })
        }

        adj
    }

    pub fn bfs(&self, source: Node, target: Node) -> Option< Vec<Node> > {
        let mut q = VecDeque::new();
        q.push_back(source);

        let mut visited: Vec<bool> = Vec::new();
        let mut parent: Vec< Option<Node> > = Vec::new();

        for _ in &self.board {
            visited.push(false);
            parent.push(None);
        }

        visited[ index(self.width, &source.point) ] = true;

        while let Some(curr) = q.pop_front() {
            if curr == target { break; }

            for point in &self.neighbours(curr.point) {
                let i = index(self.width, point);
                let next = self.board[i];

                if !visited[i] && !next.has_snake {
                    q.push_back(next);
                    visited[i] = true;
                    parent[i] = Some(curr);
                }
            }
        }

        let mut prev = target;
        let mut path = vec![target];
        
        loop {
            let i = index( self.width, &prev.point );
            match parent[i] {
                Some(node) => {
                    prev = node;
                    path.push(prev);
                },
                None => break,
            }
        }

        path.reverse();

        if path[0] == source { Some(path) }
        else { None }
    }

    pub fn connected_component(&self, source: &Node) -> Vec<Node> {
        let mut visited = Vec::new();
        for _ in &self.board {
            visited.push(false);
        }
        
        let mut cc = Vec::new();
        
        visited[ index(self.width, &source.point) ] = true;

        let mut q = VecDeque::new();
        q.push_back(*source);
        
        while let Some(curr) = q.pop_front() {
            cc.push(curr);
            
            for nb in &self.neighbours(curr.point) {
                let i = index(self.width, nb);
                let next = self.board[i];

                if !visited[i] && !next.has_snake {
                    q.push_back(next);
                    visited[i] = true;
                }
            }
        }

        cc
    }

    pub fn is_safe(&self, path: &Vec<Node>, len: usize) -> bool {
        let source = path.last().unwrap();
        let cc = self.connected_component(source);
        let result;

        if len < cc.len() {
            result = true;
        } else if cc.iter().any(|node| node.has_tail) {
            result = true;
        } else {
            result = false;
        }

        result
    }

    pub fn wait(&self, source: &Node, body: &Vec<Point>) -> Option< Vec<Node> > {
        let cc = self.connected_component(source);
        let mut target = *source;

        let mut break_outer = false;
        // find target node
        for &point in body.iter().rev() {

            for nb in &self.neighbours(point) {
                let node = self.board[ index(self.width, nb) ];
                
                if cc.contains(&node) {
                    target = node;
                    break_outer = true;
                    break;
                }
            }

            if break_outer { break }
        }
        // get shortest path to target from head neighbours
        let mut paths = Vec::new();
        for point in &self.neighbours(source.point) {
            let nb_node = self.board[ index(self.width, point) ];
            
            if cc.contains(&nb_node) {
                match self.bfs(nb_node, target) {
                    Some(path) => paths.push(path),
                    None => ()
                }
            }
        }

        paths.sort_by(|a, b| b.len().cmp(&a.len()));

        if paths.is_empty() { None }
        else { Some(paths.remove(0)) }
    }
}

pub fn index(width: usize, point: &Point) -> usize {
    width * point.y + point.x
}
