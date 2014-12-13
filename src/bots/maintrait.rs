use gamecore::{GameBoard,State};
use std::rand;

pub trait Bot {
    fn new<'a>() -> Box<Bot + 'a>;
    fn get_move(&mut self, board: &GameBoard) -> (uint, uint);
}

pub struct DummyBot {
    side: State
}

impl Bot for DummyBot {
    fn new<'a>() -> Box<Bot + 'a> {
        box DummyBot{
            side: State::O
        }
    }

    fn get_move(&mut self, board: &GameBoard) -> (uint, uint) {
        let turn = rand::random::<uint>() % 9;
        (turn / 3, turn % 3)
    }   
}