use std::io;

fn main() {
    println!("Welcome To Pick Up Sticks!");
    println!("How many sticks would you like to play with?");
    let game_size = read_number();
    let fibs = gen_fibs_upto(game_size);
    let mut sticks = game_size;
    if !fib_p(&fibs, game_size) {
        println!("I play first.");
        my_move(&fibs, sticks);
    }
}

fn my_move(fibs: &Vec<u32>, sticks: u32) {}

fn your_move(fibs: &Vec<u32>, sticks: u32) {}

fn valid_move_p(fibs: &Vec<u32>, sticks: u32) -> bool {
    true
}

fn gen_fibs_upto(game_size: u32) -> Vec<u32> {
    let mut fibs = vec![1, 2];
    if game_size <= 2 {
        return fibs;
    }
    let mut size = fibs.len();
    while fibs[size - 1] < game_size {
        fibs.push(fibs[size - 1] + fibs[size - 2]);
        size = fibs.len();
    }
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
