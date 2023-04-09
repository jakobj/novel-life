use rand::{distributions::Uniform, prelude::Distribution};

use crate::{lcell::LCell, universe::{self, Universe}};

pub fn novelty_search(
    universe_size: usize,
    seed_size: usize,
    n_ea_steps: usize,
    n_simulation_steps: usize,
) -> Vec<Vec<Vec<LCell>>> {
    fn interesting(u: &Universe) -> bool {
        universe::count_alive_cells(&u) > 20 && universe::measure_symmetry(&u) > 10
    }

    let n_offspring = 6;

    let mut discoveries = Vec::new();
    let mut archive = Vec::new();
    let mut parents = vec![
        Individual {
            cells: vec![vec![LCell::Dead; seed_size]; seed_size],
            novelty: u32::MIN
        };
        4
    ];
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..parents.len());

    for _ in 0..n_ea_steps {
        let mut offspring = Vec::with_capacity(n_offspring);
        for _ in 0..n_offspring {
            // choose a random parent and mutate
            let idx = between.sample(&mut rng);
            let cells = mutate(&parents[idx].cells, 1);

            // compute final state of universe
            let universe = Universe::new(universe_size);
            let universe = universe::seed(&universe, &cells);
            let universe = universe::simulate(&universe, n_simulation_steps);
            if interesting(&universe) {
                discoveries.push(cells.clone());
            }

            // compute novelty as total distance from four closest neighbors
            let mut distances = archive
                .iter()
                .map(|u: &Universe| universe::distance(&u, &universe))
                .collect::<Vec<u32>>();
            distances.sort_unstable();
            let novelty;
            if distances.len() < 4 {
                novelty = u32::MIN;
            } else {
                novelty = distances[..4].iter().sum::<u32>();
            }

            offspring.push(Individual { cells, novelty });
            archive.push(universe);
        }

        // choose offspring with highest novelty
        offspring.sort_by_key(|a| a.novelty);
        offspring.reverse();
        parents = offspring[..4].to_vec();
    }
    discoveries
}

#[derive(Clone)]
struct Individual {
    cells: Vec<Vec<LCell>>,
    novelty: u32,
}

fn mutate(cells: &Vec<Vec<LCell>>, n_mutations: usize) -> Vec<Vec<LCell>> {
    let mut cells = cells.clone();
    let mut rng = rand::thread_rng();
    let n = cells.len();
    let between = Uniform::from(0..n);
    for _ in 0..n_mutations {
        let row = between.sample(&mut rng);
        let col = between.sample(&mut rng);
        match cells[row][col] {
            LCell::Alive => cells[row][col] = LCell::Dead,
            LCell::Dead => cells[row][col] = LCell::Alive,
        };
    }
    cells
}
