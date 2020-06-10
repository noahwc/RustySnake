use crate::requests::{Point, Turn};

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Node {
    pub point: Point,
    pub weight: usize,
    pub has_snake: bool,
    pub has_food: bool,
    pub has_head: bool,
    pub has_tail: bool,
}

impl Node {
    pub fn new(p: Point) -> Node {
        Node {
            point: p,
            weight: 0,
            has_snake: false,
            has_food: false,
            has_head: false,
            has_tail: false,
        }
    }

    pub fn stacked(&self, t: &Turn) -> bool {
        for snake in &t.board.snakes {
            return snake.body.windows(2).any(|w| w[0] == w[1]);
        }
        false
    }
}
