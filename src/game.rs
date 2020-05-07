//  Uses
use crate::requests::{Turn, Point};
pub use std::collections::HashMap;
pub use petgraph::graph::{UnGraph, NodeIndex};

#[derive(Debug)]
pub struct Game {
    pub turn: Turn,
    pub graph: UnGraph::<i32, i32>,
    pub map: HashMap<Point, NodeIndex>,
    pub points: Vec<Point>,
    pub food_weight: i32,
}
impl Game {
    // Constructor
    pub fn new(t: Turn) -> Game {
        Game {
            turn: t,
            graph: UnGraph::new_undirected(),
            map: HashMap::new(),
            points: Vec::new(),
            food_weight: 10,
        }        
    }
    
    //Methods
    pub fn update_points(&mut self) {
        for w in 0..self.turn.board.width {
            for h in 0..self.turn.board.height {
                self.points.push(Point{x: w, y: h,});
            }
        }
    }

    pub fn update_map(&mut self) {
        for p in &self.points{
            self.map.insert(*p, self.graph.add_node(0));
        }
        for (p, i) in &self.map {
            let adj = [
            Point{x: p.x - 1, y: p.y},
            Point{x: p.x + 1, y: p.y},
            Point{x: p.x, y: p.y - 1},
            Point{x: p.x, y: p.y + 1}
            ]; 
            for n in &adj {
                match self.map.get(n) {
                    Some(adj_i) => self.graph.update_edge(*i, *adj_i, 0),
                    None => continue
                };
            }
        }
    }

    pub fn update_point_weights(&mut self) {
        for (&point, &index) in &self.map {
            if self.turn.board.food.iter().any(|&food| point == food) {
                match self.graph.node_weight_mut(index) {
                    Some(weight) => *weight = self.food_weight,
                    None => panic!("Error in update_point_weight!"),
                }
            }
        }
    }
}

