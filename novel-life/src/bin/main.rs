use std::io::Write;
use crossterm::{cursor, ExecutableCommand};

use novel_life::{individual::Individual, lcell::LCell, universe::{Universe, self}};

fn main() {
    let universe_size = 50;
    let seed_size = 5;
    let n_ea_steps = 50;
    let n_simulation_steps = 50;

    let discoveries = novelty_search(universe_size, seed_size, n_ea_steps, n_simulation_steps);

    for cells in discoveries {
        let u = Universe::new(universe_size);
        let offset = universe_size / 2 - seed_size / 2;
        let u = u.seed(&cells, offset, offset);
        let history = u.simulate_with_history(500);
        visualize(history, universe_size);
    }
}

fn novelty_search(
    universe_size: usize,
    seed_size: usize,
    n_ea_steps: usize,
    n_simulation_steps: usize,
) -> Vec<Vec<Vec<LCell>>> {
    fn interesting(u: &Universe) -> bool {
        u.n_alive() > 100
    }

    let mut discoveries = Vec::new();
    let mut archive = Vec::new();
    let mut parents = vec![Individual::new(seed_size); 4];
    for _ in 0..n_ea_steps {
        let mut offspring = parents.clone();
        for i in 0..offspring.len() {
            offspring[i] = offspring[i].mutate(1);

            let universe = Universe::new(universe_size);
            let offset = universe_size / 2 - seed_size / 2;
            let universe = universe.seed(&offspring[i].cells, offset, offset);
            let universe = universe.simulate(n_simulation_steps);
            if interesting(&universe) {
                discoveries.push(offspring[i].cells.clone());
            }

            let mut distances = archive.iter().map(|u: &Universe| u.distance(&universe)).collect::<Vec<u32>>();
            distances.sort_unstable();
            if distances.len() >= 4 {
                offspring[i].novelty = distances[..4].iter().sum::<u32>();
            }

            archive.push(universe);
        }
        let mut population = offspring.clone();
        population.append(&mut parents);
        population.sort_unstable_by_key(|a| a.novelty);

        parents = population[..4].to_vec();
    }
    discoveries
}

fn visualize(history: Vec<Universe>, universe_size: usize) {
    let mut stdout = std::io::stdout();
    let n = history.len();
    for (i, u) in history.into_iter().enumerate() {
        let s = u.to_string();
        for r in s.lines() {
            writeln!(stdout, "{}", r).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        if i != n - 1 {
            stdout.execute(cursor::MoveUp(universe_size as u16)).unwrap();
        }
    }
}
