use gamecore::{GameBoard,State};
use bots::Bot;

pub struct SimpleBot {
	side: State
}

impl Bot for SimpleBot {
	fn new(side: State) -> SimpleBot {
		SimpleBot {
			side: side
		}
	}

	fn get_move(self, board: &GameBoard) -> (uint, uint) {
		(0, 0)
	}
}