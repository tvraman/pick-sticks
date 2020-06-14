use std::{io, process::exit};
mod game;
use crate::game::Game;

fn main() {
    println!("Welcome To Pick Up Sticks!");
    println!("How many sticks would you like to play with?");

    Game::build().play();
}
