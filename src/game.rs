use std::process::exit;
#[derive(Debug)]
pub struct Game<'a> {
    pub current: u32,
    pub sticks: u32,
    pub limit: u32,
    pub fib_base: u32,
    pub last_move: u32,
    pub fibonacci: &'a Vec<u32>,
}

fn gen_fibs_upto(game_size: u32) -> Vec<u32> {
    let mut fibs = vec![1, 2];
    let mut size = fibs.len();
    while fibs[size - 1] <= game_size {
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

impl Game<'_> {
    pub const fn new(size: u32) -> Game<'static> {
        let fibs = gen_fibs_upto(size);
        Game {
            current: size,
            sticks: size,
            fib_base: fibs[fibs.len() - 2],
            limit: size - 1,
            fibonacci: &fibs,
            last_move: 0,
        }
    }

    pub fn play(&self) {
        if !fib_p(self.fibonacci, self.sticks) {
            println!("I play first.");
            self.my_move();
        }
        while self.sticks > 0 {
            self.your_move();
            self.my_move();
        }
    }
    pub fn my_move(&mut self) {
        if self.limit >= self.sticks {
            self.last_move = self.sticks;
            self.sticks = 0;
            println!("I pick {} sticks and win!", self.last_move);
            exit(0);
        }
        if self.limit >= self.current {
            self.last_move = self.current;
            self.sticks -= self.last_move;
            self.current = self.sticks;
            self.limit = 2 * self.last_move;
            println!("I pick {} sticks !", self.last_move);
            return;
        }
        println!("{:?}", self);
        while 3 * (self.current - self.fib_base) >= self.current {
            self.current = self.current - self.fib_base;
            self.update_fib_base();
        }

        self.last_move = self.current - self.fib_base;
        self.current -= self.last_move;
        self.sticks -= self.last_move;
        self.limit = 2 * self.last_move;
        if self.current == 0 {
            self.current = self.sticks;
        }
        println!("I picked {}; {} sticks left.", self.last_move, self.sticks);
        println!("{:?}", self);
    }

    pub fn your_move(&mut self) {
        println!(
            "You can pick between 1 and {} sticks; {} sticks left.",
            self.limit, self.sticks
        );
        self.last_move = super::read_number();
        self.sticks -= self.last_move;
        self.current -= self.last_move;
        self.limit = 2 * self.last_move;
        println!(
            "You picked {}; {} sticks left.",
            self.last_move, self.sticks
        );
        println!("{:?}", self);
    }

    fn update_fib_base(&mut self) {
        for f in self.fibonacci {
            if *f < self.current {
                self.fib_base = *f;
            } else {
                break;
            }
        }
    }
}
