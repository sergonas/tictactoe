use std::ops::Add;
use board::{GameBoard,State};

#[derive(Clone,Copy,PartialEq)]
pub struct Pattern { //FIXME or not
    x_count: usize,
    o_count: usize,
    empty_count: usize
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            x_count: 0usize,
            o_count: 0usize,
            empty_count: 0usize
        }
    }

    pub fn find_first(&self, board: &GameBoard) -> Option<[usize;6]> {
        match (self.vertical(board), self.horizontal(board), self.diagonal(board)) { //Does compiler optimize that?
            (x @ Some(_), _, _) | (_, x @ Some(_), _) | (_, _, x @ Some(_)) => x, //WHOA! IS BEAUTIFUL!!!
            (_, _, _) => None
        }
    }

    fn vertical(&self, board: &GameBoard) -> Option<[usize;6]> {  //FIXME
        for x in 0..3usize {
            let reslt = self.check_line(board, [0, x, 1, x, 2, x]);
            if reslt.is_some() {
                return reslt;
            }
        }
        None
    }

    fn horizontal(&self, board: &GameBoard) -> Option<[usize;6]> {
        for x in 0..3usize {
            let reslt = self.check_line(board, [x, 0, x, 1, x, 2]);
            if reslt.is_some() {
                return reslt;
            }
        }
        None
    }

    fn diagonal(&self, board: &GameBoard) -> Option<[usize;6]> {
        match (self.check_line(board, [0, 0, 1, 1, 2, 2]), self.check_line(board, [0, 2, 1, 1, 2, 0])) {
            (x @ Some(_), _) | (_, x @ Some(_)) => x,
            (_, _) => None
        }
    }

    fn check_line(&self, board: &GameBoard, line: [usize;6]) -> Option<[usize;6]> {
        let line_contains = (board.get_at((line[0], line[1])), board.get_at((line[2], line[3])), board.get_at((line[4], line[5])));
        let result = line_contains.0 * 1 + line_contains.1 * 1 + line_contains.2 * 1;
        if result == *self {
            Some(line)
        } else {
            None
        }
    }

    pub fn count(&self, state: State) -> usize {
        match state {
            State::X => self.x_count,
            State::O => self.o_count,
            State::Empty => self.empty_count
        }
    }

    pub fn set(&mut self, state: State, count: usize) { //FIXME
        match state {
            State::X => self.x_count = count,
            State::O => self.o_count = count,
            State::Empty => self.empty_count = count
        }
    }
}

impl Add for Pattern {
    type Output = Pattern;

    fn add(self, rhs: Pattern) -> Pattern {
        Pattern {
            x_count: self.x_count + rhs.x_count,
            o_count: self.o_count + rhs.o_count,
            empty_count: self.empty_count + rhs.empty_count
        }
    }
}