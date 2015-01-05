use gamecore::{GameBoard,State};
use bots::Bot;

pub struct MasterBot  {
    side: State,
    tree: Tree
}

impl Bot for MasterBot {
    fn new(side: State) -> MasterBot {
        //create tree, multithreading
        let constructed_tree = Tree;
        MasterBot {
            side: side,
            tree: constructed_tree
        }
    }

    fn get_move(&self, board: &GameBoard) -> (uint, uint) {
        (0u, 0u)
    }
}

struct Tree;