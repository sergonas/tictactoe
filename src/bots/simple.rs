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
        let can_win = self.can_win(self.side, board);
        if can_win.is_some() {
            return can_win.unwrap();
        }

        let can_opponent_win = self.can_win(!self.side, board);
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
    fn can_win(&self, check_for: State, board: &GameBoard) -> Option<(uint, uint)> {
        let pattern = check_for * 2 + State::Empty * 1;
        match pattern.find_first(board) {
            Some(x) => {
                if board.get_at((x[0], x[1])) == State::Empty {
                    Some((x[0], x[1]))
                } else if board.get_at((x[2], x[3])) == State::Empty {
                    Some((x[2], x[3]))
                } else if board.get_at((x[4], x[5])) == State::Empty {
                    Some((x[4], x[5]))
                } else {
                    panic!("Some error in pattern logic")
                }
            },
            None => None
        }
    }

    fn opposite_turn(&self, board: &GameBoard) -> (uint, uint) {
        if (board.get_at((0, 0)) == !self.side || board.get_at((1, 0)) == !self.side || 
            board.get_at((0, 1)) == !self.side) && board.get_at((2, 2)) == State::Empty {
            (2, 2)
        } else if (board.get_at((0, 2)) == !self.side || board.get_at((1, 2)) == !self.side || 
            board.get_at((0, 1)) == !self.side) && board.get_at((2, 0)) == State::Empty {
            (2, 0)
        } else if (board.get_at((2, 0)) == !self.side || board.get_at((2, 1)) == !self.side || 
            board.get_at((1, 0)) == !self.side) && board.get_at((0, 2)) == State::Empty {
            (0, 2)
        } else if (board.get_at((2, 2)) == !self.side || board.get_at((1, 0)) == !self.side || 
            board.get_at((0, 1)) == !self.side) && board.get_at((2, 2)) == State::Empty {
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