use gamecore::{GameBoard,State};
use bots::{Bot,DummyBot};

mod bots;
mod gamecore;

fn main() {
    let board = GameBoard::new();
    let mut bot: DummyBot = Bot::new(State::O);
    board.make_turn(State::X, (0, 0));
    board.make_turn(State::O, bot.get_move(&board));
}
