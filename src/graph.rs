use super::node::*;
use super::requests::*;
use super::vertex::*;
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
    where F: Fn(Node) -> (i32, bool) {
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
            adj.push(self.board[index(self.width, &Point { x: x - 1, y })])
        }
        if x < 10 {
            adj.push(self.board[index(self.width, &Point { x: x + 1, y })])
        }
        if y > 0 {
            adj.push(self.board[index(self.width, &Point { x, y: y - 1 })])
        }
        if y < 10 {
            adj.push(self.board[index(self.width, &Point { x, y: y + 1 })])
        }

        adj
    }

    pub fn djikstra(&self, source: &Node, targets: &Vec<Node>) -> Vec<Vec<Node>> {
        let mut map = HashMap::new();
        for n in &self.board {
            map.insert(
                n,
                Vertex {
                    node: n,
                    cost: 9999,
                    parent: None,
                    visited: false,
                },
            );
        }
        map.get_mut(source).unwrap().cost = 0;

        let mut pq = BinaryHeap::new();
        pq.push(*map.get(source).unwrap());

        while !pq.is_empty() {
            let curr_vertex = pq.pop().unwrap();
            map.get_mut(&curr_vertex.node).unwrap().visited = true;

            for node in &self.neighbours(&curr_vertex.node) {
                let nb_vertex = map.get_mut(node).unwrap();
                if !nb_vertex.visited {
                    if nb_vertex.cost > curr_vertex.cost + curr_vertex.node.weight {
                        nb_vertex.cost = curr_vertex.cost + curr_vertex.node.weight;
                        nb_vertex.parent = Some(*curr_vertex.node);
                        pq.push(*nb_vertex);
                    }
                }
            }
        }

        let mut paths = Vec::new();
        for &target in targets {
            let mut path = vec![target];
            loop {
                let curr_vertex = map.get(path.last().unwrap()).unwrap();
                match curr_vertex.parent {
                    Some(node) => path.push(node),
                    None => break,
                }
            }
            path.reverse();
            paths.push(path);
        }

        paths
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
                        for node in self.neighbours(&curr) {
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
