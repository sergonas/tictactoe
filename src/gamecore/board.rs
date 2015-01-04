use std::fmt;

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
}

pub enum State {
    Empty, X, O
}

impl State {
    pub fn negate(self) -> State {
        match self {
            State::O => State::X,
            State::X => State::O,
            _ => State::Empty
        }
    }     
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        match *other {
            State::Empty => {
                match *self {
                    State::Empty => true,
                    _ => false
                }
            },
            State::X => {
                match *self {
                    State::X => true,
                    _ => false
                }
            },
            State::O => {
                match *self {
                    State::O => true,
                    _ => false
                }
            }
        }
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

impl Copy for State {
    
}