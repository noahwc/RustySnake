use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> Option<responses::Move> {
    // PREREQS //
    let mut game = game::Game::new(&turn); // new game instance
    let mut paths = Vec::new();
    let head = *turn.you.body.first().expect("no head!");

    
    // EARLY GAME //
    let empty_weight = 1;
    let snake_weight = 122;
    let head_weight = 0;
    
    let weighting_heuristic = |n: &node::Node| -> (i32, bool) {
        if n.has_snake(&turn) {
            if n.point == head {
                return (head_weight, false)
            } else {
                return (snake_weight, false)
            }
        } if n.has_food(&turn) {
            return (empty_weight, true)
        }
        (empty_weight, false)
    };
    // MID GAME //

    // LATE GAME //
    
    // PATHS //
    game.graph.weight_nodes(weighting_heuristic);
    game.graph.djikstra(game.graph.get_node(&head).expect("no head in graph!"));
    for n in &game.graph.targets {
        match game.graph.path_to(n) {
            Some(path) => paths.push(path),
            None => (),
        }
    }
    paths.sort_by(|a,b| cost(a).cmp(&cost(b)));
    // ADD FLOOD FILL CHECK HERE //
    if paths.is_empty() {
        None
    } else {
        Some(responses::Move::new(get_direction(paths.first().expect("no path in paths!"))))
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