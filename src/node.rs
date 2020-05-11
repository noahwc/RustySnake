use crate::requests::{Turn, Point};

#[derive(Debug, Copy, Clone, Default)]
pub struct Node {
    pub point: Point,
    pub weight: i32,
    pub has_food: bool,
}

impl Node {
    pub fn new(t: &Turn, p: &Point, w: i32) -> Node {
        Node {
            point: *p,
            weight: w,
            has_food: t.board.food.iter().any(|n| *p == *n),
        }
    }
}
