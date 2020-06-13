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

        let mut visited: Vec<bool> = Vec::new();
        let mut parent: Vec< Option<Node> > = Vec::new();

        for _ in &self.board {
            visited.push(false);
            parent.push(None);
        }

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
        let mut visited = HashMap::<&Node, bool>::with_capacity(self.height * self.width);
        for node in &self.board {
            visited.insert(node, false);
        }
        
        let mut q = VecDeque::new();
        q.push_back(*source);

        let mut cc = Vec::new();

        while let Some(curr) = q.pop_front() {
            match visited.get(&curr) {
                Some(true) => continue,
                _ => {
                    if !curr.has_snake || curr.has_tail || curr.has_head {
                        cc.push(curr);
                        
                        for point in &self.neighbours(curr.point) {
                            let node = self.board[ index(self.width, point) ];
                            q.push_back(node)
                        }
                    }

                    if let Some(v) = visited.get_mut(&curr) {
                        *v = true;
                    }
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
        for &point in body.iter().rev() {
            for nb in &self.neighbours(point) {
                let node = self.board[ index(self.width, nb) ];
                
                if cc.contains(&node) {
                    target = node;
                    break_outer = true;
                }
            }
            if break_outer { break }
        }

        let mut paths = Vec::new();
        for point in &self.neighbours(source.point) {
            let nb_node = self.board[ index(self.width, point) ];
            
            if cc.contains(&nb_node) {
                paths.push(self.cc_bfs(&cc, nb_node, target))
            }
        }

        paths.sort_by(|a, b| b.len().cmp(&a.len()));

        if paths.is_empty() { None }
        else { Some(paths.remove(0)) }
    }

    fn cc_bfs(&self, cc: &Vec<Node>, source: Node, target: Node) -> Vec<Node> {        
        let mut visited = HashMap::<Node, bool>::with_capacity(cc.len());
        let mut parent = HashMap::<Node, Option<Node>>::with_capacity(cc.len());
        for &node in cc {
            visited.insert(node, false);
            parent.insert(node, None);
        }

        let mut q = VecDeque::new();
        q.push_back(source);

        while let Some(curr) = q.pop_front() {
            if curr == target { break }

            for nb in &self.neighbours(curr.point) {
                let next = self.board[ index(self.width, nb) ];
                
                let v = visited.get_mut(&next); 
                match v {
                    Some(false) => {
                        if cc.contains(&next) {
                            q.push_back(next);
                
                            *v.unwrap() = true;
                            
                            if let Some(p) = parent.get_mut(&next) {
                                *p = Some(curr);
                            }
                        }
                    },
                    _ => continue
                }
            }
        }

        let mut path = vec![target];
        let mut curr = &target;
        
        loop {
            if let Some(prev) = parent.get(curr).unwrap() {
                path.push(*prev);
                curr = prev;
                println!("CURR: {:#?}", curr);
            }
            else { break }
        }

        path.reverse();
        if path[0] == source { path }
        else { Vec::new() }
    }
}

pub fn index(width: usize, point: &Point) -> usize {
    width * point.y + point.x
}
