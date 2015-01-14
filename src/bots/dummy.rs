use gamecore::{GameBoard,State};
use std::rand;
use bots::Bot;

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