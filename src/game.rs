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

    fn update_fib_base(mut game: &mut Game) {
        for f in game.fibonacci {
            if *f < game.current {
                game.fib_base = *f;
            } else {
                break;
            }
        }
    }
}
