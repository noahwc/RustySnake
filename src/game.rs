use crate::requests::Turn;
use crate::board::Board;

pub struct Game<'a> {
    pub turn: &'a Turn,
    pub board: Board,
}

impl<'a> Game<'a> {
    // Constructor
    pub fn new(t: &Turn) -> Game {
        Game {
            turn: t,
            board: Board::new(),
        }        
    }
    // Methods
}