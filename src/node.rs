use crate::requests::{Turn, Point};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct Node {
    pub point: Point,
    pub weight: i8,
    pub cost: i8,
    pub parent: Option<Point>,
    pub visited: bool,
}

impl Node {
    pub fn new(p: Point, w: i8) -> Node {
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
            if snake.id == t.you.id && self.point == snake.body[0] {
                return false
            }
            return snake.body.iter().any(|&p| p == self.point)
        }
        false
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
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