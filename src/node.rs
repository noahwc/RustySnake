use crate::requests::{Turn, Point};

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Node {
    pub point: Point,
    pub weight: i32,
}

impl Node {
    pub fn new(&p: &Point, w: i32) -> Node {
        Node {
            point: p,
            weight: w,
        }
    }

    pub fn has_food(&self, t: &Turn) -> bool {
        t.board.food.iter().any(|&n| self.point == n)
    }

    pub fn has_snake(&self, t: &Turn) -> bool {
        for snake in &t.board.snakes {
            return snake.body.iter().any(|&p| p == self.point)
        }
        false
    }

    pub fn stacked(&self, t: &Turn) -> bool {
        for snake in &t.board.snakes {
            return snake.body.windows(2).any(|w| w[0] == w[1])
        }
        false
    }
}
