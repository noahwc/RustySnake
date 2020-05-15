use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    let direction = responses::Move::new(responses::Movement::Up);    // default

    let mut game = game::Game::new(&turn); // new game instance

    let weighting_heuristic = |n: &mut node::Node| {
        if n.has_snake(&turn) {
            n.weight = 122; // moving into a snake costs more than traversing every node
        }else {
            n.weight = 1; // default weight for open space
        }
    };

    game.graph.weight_nodes(weighting_heuristic);

    for food in &game.turn.board.food {
        let food_node;
        match game.graph.get_node(food){
            Some(n) => food_node = n,
            None => continue, 
        }
        match game.graph.djikstra(game.our_head(), food_node) {
            Some(path) => game.paths.push(path),
            None => continue,
        }
    }
    //
    // pursue game.best_path()

    direction   // return
}