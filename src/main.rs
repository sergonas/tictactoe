use gamecore::{State,Game};
use bots::{Bot,DummyBot};
use std::io;

mod bots;
mod gamecore;

fn main() {
    let mut game = Game::new();
    let bot: DummyBot = Bot::new(State::O);
    while game.is_game_ended() == State::Empty {
    	while !game.make_move(get_player_move()) {
    		println!("Bad move");
    	}

    	while !game.make_move_bot(&bot) { if game.is_game_ended() != State::Empty {
    			break;
    		}
    	};

    	println!("{}", game);
    }

    println!("Winner is {}!", game.is_game_ended());
}

fn get_player_move() -> (uint, uint) {
	print!("{} : ", State::X);
	let input = io::stdin().read_line().ok().unwrap();
	let res: Vec<uint> = input.as_slice().trim().split(' ').map(|x| from_str::<uint>(x).unwrap()).collect();
	(res[0] - 1, res[1] - 1) 
}
