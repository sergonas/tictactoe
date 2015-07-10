use board::{GameBoard,State};

pub trait Bot {
    fn new(side: State) -> Self;
    fn get_move(&self, board: &GameBoard) -> (usize, usize);
}