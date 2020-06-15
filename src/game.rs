use std::{io, process::exit};

#[derive(Debug)]
pub struct Game {
    current: u16,    // sub-game
    sticks: u16,     //sticks left
    limit: u16,      // max sticks for current turn
    fib_base: u16,   // Nearest Fibonacci < current,
    last_move: u16,  // most recent move
    stack: Vec<u16>, // Stack of sub-games
    fibonacci: Vec<u16>,
}
// Return a vector of Fibonacci numbers     where game_size < fibs[len-1]

fn gen_fibs_upto(game_size: u16) -> Vec<u16> {
    let mut fibs = vec![1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] < game_size {
        fibs.push(fibs[size - 1] + fibs[size - 2]);
        size = fibs.len();
    }
    fibs
}
// Predicate to check if f is a Fibonacci number:

fn fib_p(fibs: &Vec<u16>, f: u16) -> bool {
    for n in fibs {
        if f == *n {
            return true;
        }
    }
    false
}
// Read a number from stdin and return it.

fn read_number(prompt: &str) -> u16 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u16 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You didn't enter  a valid number.",);
            exit(0);
        }
    };
    input
}
// Implement methods on type Game:

impl Game {
    // Build a new game by initializing game state and needed data:

    pub fn build() -> Game {
        let size = read_number("How many sticks would you like to play with?");
        if size <= 2 {
            println!("Game size of 2  or less is too small");
            exit(1);
        }
        let fibs = gen_fibs_upto(size);
        let mut g: Game = Game {
            sticks: size,
            current: size,
            limit: size - 1,
            fib_base: fibs[fibs.len() - 2],
            last_move: 0,
            stack: Vec::new(),
            fibonacci: fibs,
        };
        if !fib_p(g.sticks) {
            g.decompose();
        }
        g
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

    // Update game state  after playing one turn:

    fn update(&mut self, pick: u16) {
        if pick > self.limit || pick == 0 {
            println!("Invalid move: {}", pick);
            println!("{:?}", self);
            exit(0);
        }
        self.last_move = pick;
        self.sticks -= pick;
        self.limit = 2 * pick;
        self.current -= pick;
        if self.current == 0 {
            self.current = match self.stack.pop() {
                Some(num) => num,
                None => self.sticks,
            };
        }
        self.update_fib_base();
        // println!("{:?}", self);
        println!("Move: {}", pick);
    }

    fn finish(&mut self) {
        self.last_move = self.sticks;
        self.sticks = 0;
        println!("I picked {} and win!", self.last_move);
        exit(0);
    }

    fn my_move(&mut self) {
        if self.limit >= self.sticks {
            self.finish();
        }

        if (self.current > 0) && (self.current <= self.limit) {
            self.update(self.current);
            return;
        } else {
            let mut next_move = self.current - self.fib_base;
            while ((3 * next_move) >= self.current) || (next_move > self.limit) {
                self.current -= next_move;
                self.update_fib_base();
                next_move = self.current - self.fib_base;
            }
            self.update(next_move);
            //println!("{:? }", self);
        }
    }

    fn your_move(&mut self) {
        println!(
            "You can pick between 1 and {}, {} sticks left.",
            self.limit, self.sticks
        );
        self.update(read_number("How many sticks do you pick?"));
        println!(
            "After picking {}, there are {} sticks left.",
            self.last_move, self.sticks
        );
        // println!("{:?}", self);
    }

    // fib_base is the  largest Fibonacci number less than current.

    fn update_fib_base(&mut self) {
        if self.current <= 2 {
            return;
        }
        for f in &self.fibonacci {
            if f < &self.current {
                self.fib_base = *f;
            } else {
                break;
            }
        }
    }

    // Decompose the game into smaller games.
    // These are pushed on to self.stack.

    fn decompose(&mut self) {
        let mut next_move = self.current - self.fib_base;
        while ((3 * next_move) >= self.current) && (self.current >= 2) {
            self.stack.push(self.fib_base);
            self.current = self.current - self.fib_base;
            self.update_fib_base();
            next_move = self.current - self.fib_base;
        }
        // reset self.current:
        self.current = self.sticks;
        // println!("{:?}", self);
    }
}
