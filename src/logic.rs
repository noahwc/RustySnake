use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    // PREREQS //
    let mut game = game::Game::new(&turn); // new game instance
    let weighting_heuristic = |n: &mut node::Node| {
        if n.has_snake(&turn) {
            n.weight = 122; // moving into a snake costs more than traversing every node
        }else {
            n.weight = 1; // default weight for open space
        }
    };
    game.graph.weight_nodes(weighting_heuristic); // weight nodes with heuristic
    game.graph.djikstra(game.graph.get_node(&game.our_head).unwrap()); // run djikstra to all nodes from head

    // EARLY GAME //
    match eat(&game) {
        Some(path) => return responses::Move::new(get_direction(&path[0], &path[1])),
        None => (),
    }
    match chase_tail(&game) {
        Some(path) => return responses::Move::new(get_direction(&path[0], &path[1])),
        None => (),
    }

    // MID GAME //

    // LATE GAME //
    

    return responses::Move::new(responses::Direction::Up)   // return default direction
}

fn eat(game: &game::Game) -> Option<Vec<node::Node>> {
    let mut paths = Vec::new();
    // get all cheapest paths to food
    for food in &game.turn.board.food {
        let food_node = game.graph.get_node(food).expect("expected food!");
        match game.graph.path_to(food_node) {
            Some(path) => paths.push(path),
            None => continue,
        }
    }

    // sort paths and return best, remove paths through snakes
    paths.sort_by(|a, b| cost(&a).cmp(&cost(&b)));
    paths.iter().filter(|p| cost(&p) < game.snake_weight).nth(0).cloned()
}

// UGLY PLS FIX
fn chase_tail(game: &game::Game) -> Option<Vec<node::Node>> {
    let tail = game.graph.get_node(&game.our_tail).expect("no tail!");
    match game.graph.path_to(tail) {
        Some(path) => {
            if cost(&path) < 2 * game.snake_weight {
                return Some(path)
            } else {
                return None
            }
        },
        None => return None
    }
}

fn get_direction(a: &node::Node, b: &node::Node) -> responses::Direction {
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