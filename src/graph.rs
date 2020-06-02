use crate::node::Node;
use crate::requests::{Point, Turn};
use std::collections::VecDeque;

pub struct Graph {
    pub width: usize,
    pub board: Vec<Node>,
    pub targets: Vec<Point>,
}

impl Graph {
    // constructor
    pub fn new(turn: &Turn) -> Graph {
        // row-major order
        let mut board = Vec::with_capacity(turn.board.width * turn.board.height);
        for row in 0..turn.board.width {
            for col in 0..turn.board.height {
                board.insert(turn.board.width * row + col, Node::new(Point {x: col, y: row}));
            }
        }
        Graph {
            width: turn.board.width,
            board: board, 
            targets: Vec::new(),
        }
    }

    // methods
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

    pub fn flood_fill(&self){

        let results: [[i8; 11]; 11];
        let mut deq = VecDeque::new();

        // init dequeue
        
        for n in &mut self.board {
            match n.point{
                Point { x, y: 0 } | Point { x: 0, y } => {
                        results[n.point.x][n.point.y] = 12;
                        for adj in self.neighbours(n.point) {
                            deq.push_back(adj);
                        }
                }
                _ =>{
                        results[n.point.x][n.point.y] = 0;
                        continue;
                }
            }
            if(n.has_snake(&turn)){
                results[n.point.x][n.point.y] = 13;
            }
            for adj in self.neighbours(n.point) {
                deq.push_back(adj);
            }
        }

        while(curr = deq.pop_front()){
            let max = 0;
            for {
                // loop thru neighbours, keep track of max score, push neighbours with scores of 0 onto deq.
            }
            // assign current node to max score
        }

        // map the results array back onto node weights (probably). This should take O(N) time.
    }

    pub fn djikstra(&mut self, start: &Point) {
        let w = self.width;
        // set start cost to 0 
        self.board[start.index(w)].cost = 0;
        // fill unvisited
        let mut unvisited: Vec<Point> = self.board.iter().map(|node| node.point).collect();

        while !unvisited.is_empty() {
            // get cheapest node
            let mut curr = unvisited[0];
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
                if !adj_node.visited {
                    if curr_cost + adj_node.weight < adj_node.cost {
                        adj_node.cost = curr_cost + adj_node.weight;
                        adj_node.parent = Some(curr);
                    }
                }
            }
            // mark curr as visited and remove from unvisited
            self.board[curr.index(w)].visited = true;
            unvisited.retain(|&point| point != curr);
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

}