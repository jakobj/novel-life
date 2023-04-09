use clap::Parser;
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;
use rand::{SeedableRng};
use rand::rngs::StdRng;

use novel_life::{
    novelty_search,
    universe::{self, Universe},
};

/// Discover interesting seeds for the Game of Life
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Size of the (square) universe
    #[arg(short, long, default_value_t = 42)]
    universe_size: usize,

    /// Size of the (square) seed
    #[arg(short, long, default_value_t = 3)]
    seed_size: usize,

    /// Number of generations for the EA
    #[arg(short, long, default_value_t = 200)]
    generations: usize,

    /// Number of simulation steps per universe
    #[arg(short, long, default_value_t = 500)]
    n_simulation_steps: usize,

    /// Seed for the rng
    #[arg(long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let mut rng = if let Some(seed) = args.seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_rng(rand::thread_rng()).unwrap()
    };

    let discoveries = novelty_search::novelty_search(
        args.universe_size,
        args.seed_size,
        args.generations,
        args.n_simulation_steps,
        &mut rng,
    );

    for cells in discoveries {
        let u = Universe::new(args.universe_size);
        let u = universe::seed(&u, &cells);
        let history = universe::simulate_with_history(&u, args.n_simulation_steps);
        visualize(history);
    }
}

fn visualize(history: Vec<Universe>) {
    let mut stdout = std::io::stdout();
    let history_len = history.len();
    for (i, u) in history.into_iter().enumerate() {
        let s = u.to_string();
        let mut n = 0;
        for r in s.lines() {
            writeln!(stdout, "{}", r).unwrap();
            n += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        if i != history_len - 1 {
            stdout.execute(cursor::MoveUp(n as u16)).unwrap();
        }
    }
}
