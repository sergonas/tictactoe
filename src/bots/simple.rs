use gamecore::{GameBoard,State};
use bots::Bot;
use std::rand;

pub struct SimpleBot {
	side: State
}

impl Bot for SimpleBot {
	fn new(side: State) -> SimpleBot {
		SimpleBot {
			side: side
		}
	}

	fn get_move(&self, board: &GameBoard) -> (uint, uint) {
        let can_win = self.can_win(board);
        if can_win.is_some() {
            return can_win.unwrap();
        }

        let can_opponent_win = self.can_opponent_win(board);
        if can_opponent_win.is_some() {
            return can_opponent_win.unwrap();
        }

        if board.get_at((1, 1)) == State::Empty {
            (1, 1)
        } else {
            self.opposite_turn(board)
        }
	}
}

impl SimpleBot {
    fn can_win(&self, board: &GameBoard) -> Option<(uint, uint)> {
        
        Option::None
    }
    
    fn can_opponent_win(&self, board: &GameBoard) -> Option<(uint, uint)> {
        let opponent = self.side.negate();

        Option::None
    }

    fn opposite_turn(&self, board: &GameBoard) -> (uint, uint) {
        if (board.get_at((0, 0)) == self.side.negate() || board.get_at((1, 0)) == self.side.negate() || 
            board.get_at((0, 1)) == self.side.negate()) && board.get_at((2, 2)) == State::Empty {
            (2, 2)
        } else if (board.get_at((0, 2)) == self.side.negate() || board.get_at((1, 2)) == self.side.negate() || 
            board.get_at((0, 1)) == self.side.negate()) && board.get_at((2, 0)) == State::Empty {
            (2, 0)
        } else if (board.get_at((2, 0)) == self.side.negate() || board.get_at((2, 1)) == self.side.negate() || 
            board.get_at((1, 0)) == self.side.negate()) && board.get_at((0, 2)) == State::Empty {
            (0, 2)
        } else if (board.get_at((2, 2)) == self.side.negate() || board.get_at((1, 0)) == self.side.negate() || 
            board.get_at((0, 1)) == self.side.negate()) && board.get_at((2, 2)) == State::Empty {
            (2, 2)
        } else {
            let mut turn = (rand::random::<uint>() % 3, rand::random::<uint>() % 3);
            while board.get_at(turn) != State::Empty {
                turn = (rand::random::<uint>() % 3, rand::random::<uint>() % 3);
            }
            turn
        }
    }
}