use super::node::*;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Vertex<'a> {
    pub node: &'a Node,
    pub cost: i32,
    pub parent: Option<Node>,
    pub visited: bool,
}

impl<'a> Ord for Vertex<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Vertex<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Vertex<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<'a> Eq for Vertex<'a> {}
