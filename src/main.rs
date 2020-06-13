use std::{io, process::exit};
mod game;
use crate::game::Game;

fn main() {
    println!("Welcome To Pick Up Sticks!");
    println!("How many sticks would you like to play with?");
    let game_size = read_number();
    if game_size <= 2 {
        println!("Game size of 2  or less is too small");
        exit(1);
    }

    Game::build(game_size).play();
}

fn read_number() -> u16 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u16 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("{}: Please enter  a  valid number", e);
            exit(0);
        }
    };
    input
}
