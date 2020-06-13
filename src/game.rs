use std::process::exit;
#[derive(Debug)]
struct Game<'a> {
    current: u32,
    sticks: u32,
    limit: u32,
    fib_base: u32,
    last_move: u32,
    fibonacci: &'a Vec<u32>,
}

impl Game {
    fn my_move(&mut self) {
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
            update_fib_base(&mut self);
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

    fn your_move(&mut self) {
        println!(
            "You can pick between 1 and {} sticks; {} sticks left.",
            self.limit, self.sticks
        );
        self.last_move = self::read_number();
        self.sticks -= self.last_move;
        self.current -= self.last_move;
        self.limit = 2 * self.last_move;
        println!(
            "You picked {}; {} sticks left.",
            self.last_move, self.sticks
        );
        println!("{:?}", game);
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
