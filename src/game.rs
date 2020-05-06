//  Uses
use crate::{requests::{Turn, Point}, responses};
use std::collections::HashMap;
use petgraph::graph::{UnGraph, NodeIndex};


pub fn direction(turn: &Turn) -> responses::Move {
    let direction = responses::Move::new(responses::Movement::Up);
    
    // Logic goes here
    let mut g = UnGraph::<i32, i32>::new_undirected();
    let mut map: HashMap<Point, NodeIndex> = HashMap::new();
    let points = all_points(&turn);
    update_map(&mut g, &mut map, &points);
    
    // Return direction
    return direction;
}

pub fn all_points(turn: &Turn) -> Vec<Point> {
    let mut points = Vec::new();
    for w in 0..turn.board.width {
        for h in 0..turn.board.height {
            points.push(Point{x: w, y: h,});
        }
    }
    return points;
}

// TODO: Write Unit Test, refactor in declarative code
pub fn update_map(g: &mut UnGraph::<i32, i32>, map: &mut HashMap<Point, NodeIndex>, points: &Vec<Point>) {
    for &p in points{
        map.insert(p, g.add_node(point_weight(&p)));
    }
    for (p, i) in map {
        let adj = [
        Point{x: p.x - 1, y: p.y},
        Point{x: p.x + 1, y: p.y},
        Point{x: p.x, y: p.y - 1},
        Point{x: p.x, y: p.y + 1},
        ]; 
        for n in &adj {
            match map.get(n) {
                Some(adj_i) => g.update_edge(*i, *adj_i, edge_weight(&p, &n)),
                None => continue
            };
        }
    }
}


fn point_weight(p: &Point) -> i32 {
    let weight: i32 = 0;
    // determine point weight here
    return weight;
}

fn edge_weight(p1: &Point, p2: &Point) -> i32 {
    let weight: i32 = 0;
    // determine edge weight here
    return weight;
}