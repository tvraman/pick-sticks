mod game;
use crate::game::Game;

fn main() {
    println!("Welcome To Pick Up Sticks!");
    Game::build().play();
}
