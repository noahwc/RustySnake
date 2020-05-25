use crate::requests::{Turn, Point};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct Node {
    pub point: Point,
    pub weight: i32,
    pub cost: i32,
    pub parent: Option<Point>,
    pub visited: bool,
}

impl Node {
    pub fn new(p: Point, w: i32) -> Node {
        Node {
            point: p,
            weight: w,
            cost: 0,
            parent: None,
            visited: false,
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

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}