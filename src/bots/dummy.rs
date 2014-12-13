use gamecore::{GameBoard,State};
use std::rand;
use bots::Bot;

pub struct DummyBot {
    side: State
}

impl Bot for DummyBot {
    fn new(side: State) -> DummyBot {
        DummyBot{
            side: side
        }
    }

    fn get_move(self, board: &GameBoard) -> (uint, uint) {
        let turn = rand::random::<uint>() % 9;
        (turn / 3, turn % 3)
    }   
}