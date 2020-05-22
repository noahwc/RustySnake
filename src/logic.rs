use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    // PREREQS //
    let mut game = game::Game::new(&turn); // new game instance
    let mut paths = Vec::new();
    let head = game.graph.get_node(turn.you.body.first().unwrap()).expect("no head");
    let tail = game.graph.get_node(turn.you.body.last().unwrap()).expect("no tail");
    let len = turn.you.body.len();

    
    // EARLY GAME //
    let empty_weight = 1;
    let snake_weight = 122;
    let food_weight = -15;
    let tail_weight = -3;
    let head_weight = 0;
    
    let weighting_heuristic = |n: &node::Node| -> i32 {
        if n.has_snake(&turn) {
            if n.point == head.point {
                return head_weight
            } if n.point == tail.point && !n.stacked(&turn) {
                return tail_weight
            } else {
                return snake_weight
            }
        } if n.has_food(&turn) {
            return food_weight
        } else {
            return empty_weight
        }
    };
    // MID GAME //

    // LATE GAME //
    
    // PATHS //
    game.graph.weight_nodes(weighting_heuristic);
    game.graph.djikstra(head);
   
    for n in &game.graph.targets {
        match game.graph.path_to(n) {
            Some(path) => paths.push(path),
            None => (),
        }
    }
    
    paths.sort_by(|a,b| cost(a).cmp(&cost(b)));
    
    // checking if next move puts us in a dead end
    match paths.iter().find(|&path| len > game.graph.flood_fill(&vec![path[1]], &turn)) {
        Some(path) => return responses::Move::new(get_direction(path)),
        None => return responses::Move::new(responses::Direction::Up)
    }
}

fn get_direction(path: &Vec<node::Node>) -> responses::Direction {
    let a = path.first().expect("no first node!");
    let b = path.iter().nth(1).expect("no second node!");
    if b.point.x > a.point.x {
        return responses::Direction::Right
    } if b.point.x < a.point.x {
        return responses::Direction::Left
    } if b.point.y > a.point.y {
        return responses::Direction::Down
    } else {
        return responses::Direction::Up
    }
}

pub fn cost(v: &Vec<node::Node>) -> i32 {
    let mut sum: i32 = 0;
    v.iter().for_each(|n| sum += n.weight);
    sum
}