use std::process::exit;
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
    let mut fibs = vec![1, 1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] <= game_size {
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

impl Game {
    pub fn build(size: u16) -> Game {
        let fibs = gen_fibs_upto(size);
        Game {
            sticks: size,
            fib_base: fibs[fibs.len() - 2],
            current: size - fibs[fibs.len() - 2],
            limit: size - 1,
            stack: Vec::new(),
            fibonacci: fibs,
            last_move: 0,
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
        println!("{:?}", self);
        if self.limit >= self.sticks {
            self.last_move = self.sticks;
            self.sticks = 0;
            println!("I pick {} sticks and win!", self.last_move);
            exit(0);
        }
        self.update_fib_base();
        let mut next_move = self.current - self.fib_base;
        while ((3 * next_move) >= self.current) || (next_move > self.limit) {
            self.stack.push(self.current);
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
        self.last_move = super::read_number();
        self.sticks -= self.last_move;
        self.current -= self.last_move;
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
            if f <= &self.current {
                self.fib_base = *f;
            } else {
                break;
            }
        }
    }
}
