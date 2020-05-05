// Modules
use crate::requests;
pub use std::collections::HashMap;

// Crates
pub use petgraph::graph::{UnGraph, NodeIndex};
pub use petgraph::dot::Dot;

pub fn new (turn: &requests::Turn) -> (HashMap<(i32, i32), NodeIndex>, UnGraph<i32, i32>) {
    let mut g = UnGraph::<i32, i32>::new_undirected();
    let mut node_map = HashMap::new();
    
    for x in 0..turn.board.width {
        for y in 0..turn.board.height {
            let node_weight: i32 = 0;
            node_map.insert((x, y), g.add_node(node_weight));
        }
    }

    for (node, index) in &node_map {
        let x = node.0;
        let y = node.1;
        let adj = [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)];
        let weight: i32 = 0;

        for n in &adj {
            match node_map.get(n) {
                Some(adj_index) => g.update_edge(*index, *adj_index, weight),
                None => continue
            };
        }
    }

    return (node_map, g)
}