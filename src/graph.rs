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

    pub fn neighbours(&self, source: &Node) -> Vec<Point> {
        let x = source.point.x;
        let y = source.point.y;
        let mut adj = Vec::new();

        if x > 0 {
            adj.push(Point { x: x - 1, y })
        }
        if x < 10 {
            adj.push(Point { x: x + 1, y })
        }
        if y > 0 {
            adj.push(Point { x, y: y - 1 })
        }
        if y < 10 {
            adj.push(Point { x, y: y + 1 })
        }

        adj
    }

    pub fn bfs(&self, source: Node, target: Node) -> Option< Vec<Node> > {
        let mut q = VecDeque::new();
        q.push_back(source);

        let mut visited: Vec<bool> = self.board
            .iter()
            .map(|_| false)
            .collect();

        visited[ index( self.width, &source.point) ] = true;

        let mut parent: Vec< Option<Node> > = self.board
            .iter()
            .map(|_| None)
            .collect();

        while let Some(curr) = q.pop_front() {
            if curr == target { break; }

            for point in &self.neighbours(&curr) {
                let i = index(self.width, point);
                let next = self.board[i];

                if !visited[i] && !next.has_snake {
                    q.push_back(next);
                    visited[i] = true;
                    parent[i] = Some(curr);
                }
            }
        }

        let mut path = Vec::new();
        let prev = target;

        while let Some(prev) = parent[ index(self.width, &prev.point) ] {
            path.push(prev);
        }

        path.reverse();

        if path[0] == source { return Some(path); }
        else {return None}
    }

    pub fn connected_component(&self, &source: &Node) -> Vec<Node> {
        let mut cc = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::<Node>::with_capacity(self.height * self.width);
        queue.push_back(source);

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            match visited.get(&curr) {
                Some(_p) => continue,
                None => {
                    if !curr.has_snake || curr.has_tail {
                        cc.push(curr);
                        for point in &self.neighbours(&curr) {
                            let node = self.board[ index( self.width, point )];
                            queue.push_back(node)
                        }
                    }
                    visited.insert(curr);
                }
            }
        }

        return cc;
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
}

pub fn index(width: usize, point: &Point) -> usize {
    width * point.y + point.x
}
