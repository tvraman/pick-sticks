use std::{io, process::exit};

#[derive(Debug)]
struct Game<'a> {
    size: u32,
    sticks: u32,
    limit: u32,
    fib_base: u32,
    last_move: u32,
    fibonacci: &'a Vec<u32>,
}

fn main() {
    println!("Welcome To Pick Up Sticks!");
    println!("How many sticks would you like to play with?");
    let game_size = read_number();
    if game_size <= 2 {
        println!("Game size of 2  or less is too small");
        exit(1);
    }
    let fibs = gen_fibs_upto(game_size);

    let mut game = Game {
        size: game_size,
        sticks: game_size,
        fib_base: fibs[fibs.len() - 1],
        limit: game_size - 1,
        fibonacci: &fibs,
        last_move: 0,
    };

    if !fib_p(&fibs, game.size) {
        println!("I play first.");
        my_move(&mut game);
    }
    while game.sticks > 0 {
        your_move(&mut game);
        my_move(&mut game);
    }
}

fn my_move(mut game: &mut Game) {
    while 3 * (game.sticks - game.fib_base) >= game.sticks {
        let gap = game.sticks - game.fib_base;
        update_fib_base(&mut game, gap);
        println!("fib_base:{}", game.fib_base);
    }

    game.last_move = game.sticks - game.fib_base;
    game.sticks -= game.last_move;
    game.limit = 2 * game.last_move;
    println!("{:?}", game);
    exit(0);
}

fn your_move(game: &mut Game) {
    game.sticks;
}

fn gen_fibs_upto(game_size: u32) -> Vec<u32> {
    let mut fibs = vec![1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] < game_size {
        let mut next = fibs[size - 1] + fibs[size - 2];
        if next < game_size {
            fibs.push(next);
            size = fibs.len();
        } else {
            break;
        }
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

fn update_fib_base(game: &mut Game, sticks: u32) {
    for f in game.fibonacci {
        if f < &sticks {
            game.fib_base = *f;
        } else {
            break;
        }
    }
}
