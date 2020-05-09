use crate::requests::{Turn, Point};

#[derive(Debug, Copy, Clone, Default)]
pub struct Node {
    weight: i32,
    has_food: bool,
}

impl Node {
    pub fn new(t: &Turn, p: &Point, w: i32) -> Node {
        Node {
            weight: w,
            has_food: t.board.food.iter().any(|n| *p == *n),
        }
    }

    pub fn update_weight<F>(&mut self, heuristic: F) where F: Fn() -> i32 {
        self.weight = heuristic();
    }
}
