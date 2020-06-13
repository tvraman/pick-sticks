use std::io;
use std::process::exit;

mod game;

fn main() {
    println!("Welcome To Pick Up Sticks!");
    println!("How many sticks would you like to play with?");
    let game_size = read_number();
    if game_size <= 2 {
        println!("Game size of 2  or less is too small");
        exit(1);
    }
    let fibs = gen_fibs_upto(game_size);

    let mut game = game::Game {
        current: game_size,
        sticks: game_size,
        fib_base: fibs[fibs.len() - 2],
        limit: game_size - 1,
        fibonacci: &fibs,
        last_move: 0,
    };

    if !fib_p(&fibs, game.sticks) {
        println!("I play first.");
        game.my_move();
    }
    while game.sticks > 0 {
        game.your_move();
        game.my_move();
    }
}

fn gen_fibs_upto(game_size: u32) -> Vec<u32> {
    let mut fibs = vec![1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] < game_size {
        fibs.push(fibs[size - 1] + fibs[size - 2]);
        size = fibs.len();
    }
    println!("fibs: {:?}", fibs);
    fibs
}

fn fib_p(fibs: &Vec<u32>, f: u32) -> bool {
    for n in fibs {
        if f == *n {
            return true;
        }
    }
    false
}

fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}: Please enter  a  valid number", e);
            0
        }
    };
    input
}
