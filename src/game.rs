//  Uses
use crate::requests::{Turn, Point};

#[derive(Debug)]
pub struct Game {
    pub turn: Turn,
    pub points: Vec<Point>,
}
impl Game {
    // Constructor
    pub fn new(t: Turn) -> Game {
        Game {
            turn: t,
            points: Vec::new(),
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
}

pub struct Node {
    point: Point,
    weight: i32,
}
impl Node {
    pub fn new(p: Point, w: i32) -> Node {
        Node {
            point: p,
            weight: w,
        }
    }

    pub fn update_weight<F>(&mut self, heuristic: F) where F: Fn(Point) -> i32 {
        self.weight = heuristic(self.point);
    }
}