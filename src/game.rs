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

impl Game<'_> {
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
