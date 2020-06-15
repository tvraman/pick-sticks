mod game;
use game::Game;

fn main() {
  println!("Welcome To Pick Up Sticks!");
  Game::build().play();
}
