extern crate rand;
extern crate gamecore;
use gamecore::{GameBoard,State,Bot};

pub struct DummyBot;

impl Bot for DummyBot {
    fn new(side: State) -> DummyBot {
        DummyBot
    }

    fn get_move(&self, board: &GameBoard) -> (usize, usize) {
        let turn = rand::random::<usize>() % 9;
        (turn / 3, turn % 3)
    }
}