#![feature(associated_types)]
#![feature(default_type_params)]
use gamecore::{State,Game};
use bots::{Bot,DummyBot,SimpleBot};
use std::io;

mod bots;
mod gamecore;

fn main() {
    let mut game = Game::new();
    let bot1: SimpleBot = Bot::new(State::X);
    let bot2: DummyBot = Bot::new(State::O);
    println!("{}", game);

    let mut counter = 1i;
    while game.is_game_ended() == None {
        println!("{}zzzzzzzzzzzzzzzzzzzzzzz", counter);
        counter += 1;
        while !game.make_move_bot(&bot1) && game.is_game_ended() == None { };
        println!("{}", game);

        println!("{}zzzzzzzzzzzzzzzzzzzzzzz", counter);
        counter += 1;
        while !game.make_move_bot(&bot2) && game.is_game_ended() == None { };

        println!("{}", game);
    }
    match game.is_game_ended() {
        Some(State::Empty) => println!("Draw!"),
        Some(x) => println!("Winner is {}", x),
        None => println!("Something stange happend =(")
    }
}

fn get_player_move() -> (uint, uint) {
	print!("{} : ", State::X);
	let input = io::stdin().read_line().ok().unwrap();
	let res: Vec<uint> = input.as_slice().trim().split(' ').map(|x| x.parse::<uint>().unwrap()).collect();
	(res[0] - 1, res[1] - 1) 
}
