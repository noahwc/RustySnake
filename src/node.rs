use crate::requests::{Turn, Point};

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Node {
    pub point: Point,
    pub weight: i32,
    pub cost: i32,
    pub parent: Option<Point>,
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
            cost: 9999,
            parent: None,
            has_snake: false,
            has_food: false,
            has_head: false,
            has_tail: false,
        }
    }

    pub fn stacked(&self, t: &Turn) -> bool {
        for snake in &t.board.snakes {
            return snake.body.windows(2).any(|w| w[0] == w[1])
        }
        false
    }
}
