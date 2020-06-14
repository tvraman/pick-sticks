use std::{io, process::exit};

#[derive(Debug)]
pub struct Game {
    current: u16,
    sticks: u16,
    limit: u16,
    fib_base: u16,
    last_move: u16,
    stack: Vec<u16>,
    fibonacci: Vec<u16>,
}

fn gen_fibs_upto(game_size: u16) -> Vec<u16> {
    let mut fibs = vec![1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] < game_size {
        fibs.push(fibs[size - 1] + fibs[size - 2]);
        size = fibs.len();
    }
    fibs
}

fn fib_p(fibs: &Vec<u16>, f: u16) -> bool {
    for n in fibs {
        if f == *n {
            return true;
        }
    }
    false
}

fn read_number() -> u16 {
    println!("Pick number of sticks:");
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

impl Game {
    // Build a new game by initializing game state and needed data:

    pub fn build() -> Game {
        let size = read_number();
        if size <= 2 {
            println!("Game size of 2  or less is too small");
            exit(1);
        }
        let fibs = gen_fibs_upto(size);
        Game {
            sticks: size,
            current: size,
            limit: size - 1,
            fib_base: fibs[fibs.len() - 2],
            last_move: 0,
            stack: Vec::new(),
            fibonacci: fibs,
        }
    }

    pub fn play(&mut self) {
        if !fib_p(&self.fibonacci, self.sticks) {
            println!("I play first.");
            self.my_move();
        }
        while self.sticks > 0 {
            self.your_move();
            self.my_move();
        }
    }

    fn my_move(&mut self) {
        if self.stack.is_empty() {
            self.decompose();
        }
        if self.limit >= self.sticks {
            self.last_move = self.sticks;
            self.sticks = 0;
            println!("I pick {} sticks and win!", self.last_move);
            exit(0);
        }
        if self.current <= 2 {
            self.last_move = 2;
            self.sticks -= self.last_move;
            self.limit = 2 * self.last_move;
            println!("I pick {} sticks", self.last_move);
            return;
        }
        if self.current == 0 {
            self.current = match self.stack.pop() {
                Some(num) => num,
                None => self.sticks,
            };
        }
        if self.current <= self.limit {
            self.last_move = self.current;
            self.limit = 2 * self.last_move;
            self.sticks -= self.last_move;
            println!("I picked {} sticks", self.last_move);
            return;
        }
        let mut next_move = self.current - self.fib_base;
        while ((3 * next_move) >= self.current) || (next_move > self.limit) {
            self.current -= next_move;
            self.update_fib_base();
            next_move = self.current - self.fib_base;
        }
        println!("{:?}", self);
        self.last_move = next_move;
        self.current -= self.last_move;
        self.sticks -= self.last_move;
        self.limit = 2 * self.last_move;
        println!("I picked {}; {} sticks left.", self.last_move, self.sticks);
        println!("{:?}", self);
    }

    fn your_move(&mut self) {
        println!(
            "You can pick between 1 and {} sticks; {} sticks left.",
            self.limit, self.sticks
        );
        self.last_move = read_number();
        self.sticks -= self.last_move;
        self.limit = 2 * self.last_move;
        self.update_fib_base();
        println!(
            "You picked {}; {} sticks left.",
            self.last_move, self.sticks
        );
        println!("{:?}", self);
    }
    // fib_base is the  largest Fibonacci number less than current.

    fn update_fib_base(&mut self) {
        for f in &self.fibonacci {
            if f < &self.current {
                self.fib_base = *f;
            } else {
                break;
            }
        }
    }
    // Decompose the game into smaller games.
    // These are pushed on to game.stack.

    pub fn decompose(&mut self) {
        while 2 * self.current > self.fib_base {
            self.stack.push(self.fib_base);
            self.current = self.current - self.fib_base;
            self.update_fib_base();
        }
        println!("{:?}", self);
    }
}
