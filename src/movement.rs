// File could use a rename for clarity, I couldn't think of anything better
use crate::{responses, requests, game};

pub fn get_move (turn: requests::Turn) -> responses::Move {
    let direction = responses::Move::new(responses::Movement::Up);    // default

    //Logic goes here
    let game = game::Game::new(turn);

    direction   // return
}