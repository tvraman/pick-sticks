use std::io;
use std::process::exit;

// mod game;
#[derive(Debug)]
struct Game<'a> {
    current: u32,
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
        current: game_size,
        sticks: game_size,
        fib_base: fibs[fibs.len() - 2],
        limit: game_size - 1,
        fibonacci: &fibs,
        last_move: 0,
    };

    if !fib_p(&fibs, game.sticks) {
        println!("I play first.");
        my_move(&mut game);
    }
    while game.sticks > 0 {
        your_move(&mut game);
        my_move(&mut game);
    }
}

fn my_move(mut game: &mut Game) {
    if game.limit >= game.sticks {
        game.last_move = game.sticks;
        game.sticks = 0;
        println!("I pick {} sticks and win!", game.last_move);
        exit(0);
    }
    if game.limit >= game.current {
        game.last_move = game.current;
        game.sticks -= game.last_move;
        game.current = game.sticks;
        game.limit = 2 * game.last_move;
        println!("I pick {} sticks !", game.last_move);
        return;
    }
    println!("{:?}", game);
    while 3 * (game.current - game.fib_base) >= game.current {
        game.current = game.current - game.fib_base;
        update_fib_base(&mut game);
    }

    game.last_move = game.current - game.fib_base;
    game.current -= game.last_move;
    game.sticks -= game.last_move;
    game.limit = 2 * game.last_move;
    if game.current == 0 {
        game.current = game.sticks;
    }
    println!("I picked {}; {} sticks left.", game.last_move, game.sticks);
    println!("{:?}", game);
}

fn your_move(game: &mut Game) {
    println!(
        "You can pick between 1 and {} sticks; {} sticks left.",
        game.limit, game.sticks
    );
    game.last_move = read_number();
    game.sticks -= game.last_move;
    game.current -= game.last_move;
    game.limit = 2 * game.last_move;
    println!(
        "You picked {}; {} sticks left.",
        game.last_move, game.sticks
    );
    println!("{:?}", game);
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

fn update_fib_base(mut game: &mut Game) {
    for f in game.fibonacci {
        if *f < game.current {
            game.fib_base = *f;
        } else {
            break;
        }
    }
}
