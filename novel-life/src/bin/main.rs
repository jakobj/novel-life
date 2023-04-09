use crossterm::{cursor, ExecutableCommand};
use std::io::Write;

use novel_life::{novelty_search, universe::{self, Universe}};

fn main() {
    let universe_size = 52;
    let seed_size = 3;
    let n_ea_steps = 200;
    let n_simulation_steps = 500;

    let discoveries =
        novelty_search::novelty_search(universe_size, seed_size, n_ea_steps, n_simulation_steps);

    for cells in discoveries {
        let u = Universe::new(universe_size);
        let u = universe::seed(&u, &cells);
        let history = universe::simulate_with_history(&u, n_simulation_steps);
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
