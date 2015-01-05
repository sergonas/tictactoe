use std::fmt;
use std::ops::{Not,Mul};
use gamecore::Pattern;

pub struct GameBoard {
    field: [[State;3];3]
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            field: [[State::Empty;3];3]
        }
    }

    pub fn set_at(& mut self, who: State, at: (uint, uint)) {
        self.field[at.0][at.1] = who;
    }

    pub fn get_at(&self, at: (uint, uint)) -> State {
        self.field[at.0][at.1]
    }

    pub fn empty_count(&self) -> uint {
        let mut counter = 0u;
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

#[derive(Copy,PartialEq)]
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

impl Mul<uint> for State {
    type Output = Pattern;

    fn mul(self, rhs: uint) -> Pattern {
        let mut p = Pattern::new();
        p.set(self, rhs);
        p
    }
}

impl fmt::Show for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            State::Empty => "_",
            State::X => "X",
            State::O => "O"
        };
        fmt.pad(out)
    }
}