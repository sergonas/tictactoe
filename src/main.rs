#![allow(dead_code,unused_variables,unstable)]
use gamecore::{State,Game};
use bots::{Bot,DummyBot,SimpleBot};
use std::io;

mod bots;
mod gamecore;

// fn main() {
//     let mut results = [0u64;3];
//     for _ in 0..1000000 {
//         let mut game = Game::new();
//         let bot1: SimpleBot = Bot::new(State::X);
//         let bot2: DummyBot = Bot::new(State::O);
//         while game.is_game_ended() == None {
//             while !game.make_move_bot(&bot1) && game.is_game_ended() == None { };
//             while !game.make_move_bot(&bot2) && game.is_game_ended() == None { };
//         }
//         match game.is_game_ended() {
//             Some(State::Empty) => results[0] += 1,
//             Some(State::X) => results[1] += 1,
//             Some(State::O) => results[2] += 1,
//             None => println!("Something stange happend =(")
//         }
//     }

//     println!("Draw: {}%", results[0] as f64 / 10000f64);
//     println!("SimpleBot wins: {}%", results[1] as f64  / 10000f64);
//     println!("DummyBot wins: {}%", results[2] as f64  / 10000f64);
// }

fn main() {
    let mut game = Game::new();
    let bot: SimpleBot = Bot::new(State::O);
    while game.is_game_ended().is_none() {
        println!("{:?}", game);
        while !game.make_move(get_player_move()) {
            println!("Bad move");
        }
        if game.is_game_ended().is_some() { break; };
        while !game.make_move_bot(&bot) { };
    }
    match game.is_game_ended() {
        Some(State::Empty) => println!("Draw"),
        Some(State::X) => println!("You win"),
        Some(State::O) => println!("You lose"),
        None => println!("Something stange happend =(")
    }
}

fn get_player_move() -> (usize, usize) {
	print!("{:?} : ", State::X);
	let input = io::stdin().read_line().ok().unwrap();
	let res: Vec<usize> = input.as_slice().trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
	(res[0] - 1, res[1] - 1) 
}
