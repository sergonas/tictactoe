use gamecore::{GameBoard,State};
use bots::Bot;
use std::fmt;

pub struct Game {
    board: GameBoard,
    last_turn: State
}

impl Game {
	pub fn new() -> Game {
		Game {
			board: GameBoard::new(),
			last_turn: State::O
		}
	}

	pub fn make_move(&mut self, point: (uint, uint)) -> bool {
		if self.board.get_at(point) == State::Empty && self.is_game_ended() == State::Empty {
			self.board.set_at(self.last_turn.negate(), point);
			self.last_turn = self.last_turn.negate();
			true
		} else {
			false
		}
	}

	pub fn make_move_bot<T: Bot + Copy>(&mut self, bot: &T) -> bool {
		let move_to = bot.get_move(&self.board);
		self.make_move(move_to)
	}

	pub fn is_game_ended(&self) -> State {
		if self.check_for_winner(State::O) {
			State::O
		} else if self.check_for_winner(State::X) {
			State::X
		} else {
			State::Empty
		}
	}

	fn check_for_winner(&self, side: State) -> bool {
		(self.board.get_at((0, 0)) == side && self.board.get_at((1, 0)) == side && self.board.get_at((2, 0)) == side) ||
		(self.board.get_at((0, 1)) == side && self.board.get_at((1, 1)) == side && self.board.get_at((2, 1)) == side) ||
		(self.board.get_at((0, 2)) == side && self.board.get_at((1, 2)) == side && self.board.get_at((2, 2)) == side) ||
		 
		// По вертикали
		(self.board.get_at((0, 0)) == side && self.board.get_at((0, 1)) == side && self.board.get_at((0, 2)) == side) ||
		(self.board.get_at((1, 0)) == side && self.board.get_at((1, 1)) == side && self.board.get_at((1, 2)) == side) ||
		(self.board.get_at((2, 0)) == side && self.board.get_at((2, 1)) == side && self.board.get_at((2, 2)) == side) ||
		 
		// По диагонали
		(self.board.get_at((0, 0)) == side && self.board.get_at((1, 1)) == side && self.board.get_at((2, 2)) == side) ||
		(self.board.get_at((2, 0)) == side && self.board.get_at((1, 1)) == side && self.board.get_at((0, 2)) == side)
	}
}

impl fmt::Show for Game {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{} {} {}\n{} {} {}\n{} {} {}", self.board.get_at((0,0)), self.board.get_at((0,1)), 
			self.board.get_at((0,2)), self.board.get_at((1,0)), self.board.get_at((1,1)), self.board.get_at((1,2)), 
			self.board.get_at((2,0)), self.board.get_at((2,1)), self.board.get_at((2,2)))
	}
} 