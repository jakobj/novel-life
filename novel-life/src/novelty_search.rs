use std::collections::HashSet;

use rand::{distributions::Uniform, prelude::Distribution, rngs::StdRng};

use crate::{
    lcell::LCell,
    universe::{self, Universe},
};

pub fn novelty_search(
    universe_size: usize,
    seed_size: usize,
    generations: usize,
    n_simulation_steps: usize,
    rng: &mut StdRng,
) -> Vec<Vec<Vec<LCell>>> {
    fn interesting(u: &Universe) -> bool {
        universe::count_alive_cells(u) > 20 && universe::measure_symmetry(u) > 10
    }

    let n_parents = 4;
    let n_offspring = 6;
    let n_mutations = 1;
    let k_nearest_neighbors = 4;

    let mut discoveries = Vec::new();
    // we use a hashset as the archive, this gets the density wrong but
    // accelerates the search; since the "behavioural" space is anyway large,
    // the error in the estimate should not be too relevant
    let mut archive = HashSet::new();
    let mut parents = vec![
        Individual {
            cells: vec![vec![LCell::Dead; seed_size]; seed_size],
            novelty: usize::MIN
        };
        n_parents
    ];
    let between = Uniform::from(0..parents.len());

    for _ in 0..generations {
        let mut offspring = Vec::with_capacity(n_offspring);
        for _ in 0..n_offspring {
            // choose a random parent and mutate
            let idx = between.sample(rng);
            let cells = mutate(&parents[idx].cells, n_mutations, rng);

            // compute final state of universe
            let universe = Universe::new(universe_size);
            let universe = universe::seed(&universe, &cells);
            let universe = universe::simulate(&universe, n_simulation_steps);
            if interesting(&universe) {
                discoveries.push(cells.clone());
            }

            // compute novelty as total distance from k closest neighbors
            let mut distances = archive
                .iter()
                .map(|u| universe::compute_distance(u, &universe))
                .collect::<Vec<usize>>();
            distances.sort_unstable();
            let novelty = if distances.len() < k_nearest_neighbors {
                usize::MIN
            } else {
                distances[..k_nearest_neighbors].iter().sum::<usize>()
            };

            offspring.push(Individual { cells, novelty });
            archive.insert(universe);
        }

        // choose offspring with highest novelty
        offspring.sort_by_key(|a| a.novelty);
        offspring.reverse();
        parents = offspring[..n_parents].to_vec();
    }
    discoveries
}

#[derive(Clone)]
struct Individual {
    cells: Vec<Vec<LCell>>,
    novelty: usize,
}

fn mutate(cells: &[Vec<LCell>], n_mutations: usize, rng: &mut StdRng) -> Vec<Vec<LCell>> {
    let mut cells = cells.to_vec();
    let n = cells.len();
    let between = Uniform::from(0..n);
    for _ in 0..n_mutations {
        let row = between.sample(rng);
        let col = between.sample(rng);
        match cells[row][col] {
            LCell::Alive => cells[row][col] = LCell::Dead,
            LCell::Dead => cells[row][col] = LCell::Alive,
        };
    }
    cells
}
