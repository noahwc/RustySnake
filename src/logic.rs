use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    let mut game = game::Game::new(&turn); // new game instance

    let weighting_heuristic = |n: &mut node::Node| {
        if n.has_snake(&turn) {
            n.weight = 122; // moving into a snake costs more than traversing every node
        }else {
            n.weight = 1; // default weight for open space
        }
    };

    game.graph.weight_nodes(weighting_heuristic); // weight nodes with heuristic

    game.graph.djikstra(game.our_head); // run djikstra for all nodes

    // get all cheapest paths to food
    for food in &game.turn.board.food {
        let food_node = game.graph.get_node(food).unwrap();
        match game.graph.path_to(food_node) {
            Some(path) => game.paths.push(path),
            None => continue,
        }
    }
    
    // pursue best path
    let best_path = game.best_path();
    //println!("DESTINATION: {:?}", best_path.last());
    let direction = get_direction(&best_path[0], &best_path[1]);

    responses::Move::new(direction)   // return
}

fn get_direction(a: &node::Node, b: &node::Node) -> responses::Direction {
    let mut d = responses::Direction::Up;
    if b.point.x > a.point.x {
        d = responses::Direction::Right;
    }
    if b.point.x < a.point.x {
        d = responses::Direction::Left;
    }
    if b.point.y > a.point.y {
        d = responses::Direction::Down;
    }
    if b.point.y < a.point.y {
        d = responses::Direction::Up;
    }
    d //return
}