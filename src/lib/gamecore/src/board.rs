use std::fmt;
use std::ops::{Not,Mul};
use pattern::Pattern;

pub struct GameBoard {
    field: [[State;3];3]
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            field: [[State::Empty;3];3]
        }
    }

    pub fn set_at(& mut self, who: State, at: (usize, usize)) {
        self.field[at.0][at.1] = who;
    }

    pub fn get_at(&self, at: (usize, usize)) -> State {
        self.field[at.0][at.1]
    }

    pub fn empty_count(&self) -> usize {
        let mut counter = 0usize;
        for i in self.field.iter() {
            for j in i.iter() {
                match *j {
                    State::Empty => counter += 1,
                    _ => {},
                }
            }
        }

        counter
    }
}

#[derive(Copy,Clone,PartialEq)]
pub enum State {
    Empty, X, O
}

impl Not for State {
    type Output = State;

    fn not(self) -> State {        
        match self {
            State::X => State::O,
            State::O => State::X,
            State::Empty => State::Empty
        }
    }
}

impl Mul<usize> for State {
    type Output = Pattern;

    fn mul(self, rhs: usize) -> Pattern {
        let mut p = Pattern::new();
        p.set(self, rhs);
        p
    }
}

impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            State::Empty => "_",
            State::X => "X",
            State::O => "O"
        };
        fmt.pad(out)
    }
}