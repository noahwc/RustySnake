use crate::{responses, requests, game, node};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    let direction = responses::Move::new(responses::Movement::Up);    // default

    let mut game = game::Game::new(&turn); // new game instance

    let weighting_heuristic = |n: &mut node::Node| {
        if n.has_snake(&turn) {
            n.weight = 122; // moving into a snake costs more than traversing every node
        }else {
            n.weight = 0; // default weight for open space
        }
    };

    game.board.weight_nodes(weighting_heuristic);

    direction   // return
}