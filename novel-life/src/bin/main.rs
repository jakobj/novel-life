use crossterm::{cursor, ExecutableCommand};
use rand::{distributions::Uniform, prelude::Distribution};
use std::io::Write;

use novel_life::{
    individual::Individual,
    lcell::LCell,
    universe::Universe,
};

fn main() {
    let universe_size = 52;
    let seed_size = 3;
    let n_ea_steps = 200;
    let n_simulation_steps = 500;

    let discoveries = novelty_search(universe_size, seed_size, n_ea_steps, n_simulation_steps);

    for cells in discoveries {
        let u = Universe::new(universe_size);
        let offset = universe_size / 2 - seed_size / 2;
        let u = u.seed(&cells, offset, offset);
        let history = u.simulate_with_history(n_simulation_steps);
        visualize(history);
    }
}

fn novelty_search(
    universe_size: usize,
    seed_size: usize,
    n_ea_steps: usize,
    n_simulation_steps: usize,
) -> Vec<Vec<Vec<LCell>>> {
    fn interesting(u: &Universe) -> bool {
        u.n_alive() > 20 && u.symmetry() > 10
    }

    let n_offspring = 6;

    let mut discoveries = Vec::new();
    let mut archive = Vec::new();
    let mut parents = vec![Individual::new(seed_size); 4];
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..parents.len());
    for _ in 0..n_ea_steps {
        let mut offspring = Vec::with_capacity(n_offspring);

        for _ in 0..n_offspring {

            // choose a random parent and mutate
            let idx = between.sample(&mut rng);
            let mut ind = parents[idx].mutate(1);

            // compute final state of universe
            let universe = Universe::new(universe_size);
            let offset = universe_size / 2 - seed_size / 2;
            let universe = universe.seed(&ind.cells, offset, offset);
            let universe = universe.simulate(n_simulation_steps);
            if interesting(&universe) {
                discoveries.push(ind.cells.clone());
            }

            // compute novelty as total distance from four closest neighbors
            let mut distances = archive
                .iter()
                .map(|u: &Universe| u.distance(&universe))
                .collect::<Vec<u32>>();
            distances.sort_unstable();
            if distances.len() >= 4 {
                ind.novelty = distances[..4].iter().sum::<u32>();
            }

            offspring.push(ind);
            archive.push(universe);
        }

        // choose offspring with highest novelty
        offspring.sort_by_key(|a| a.novelty);
        offspring.reverse();
        parents = offspring[..4].to_vec();
    }
    discoveries
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
