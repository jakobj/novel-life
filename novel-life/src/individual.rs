use rand::{distributions::Uniform, prelude::Distribution};

use crate::lcell::LCell;

#[derive(Clone)]
pub struct Individual {
    pub cells: Vec<Vec<LCell>>,
    pub novelty: u32,
}

impl Individual {
    pub fn new(size: usize) -> Self {
        Self{ cells: vec![vec![LCell::Dead; size]; size], novelty: u32::MIN}
    }

    pub fn mutate(&self, n_mutations: usize) -> Self {
        let mut ind = self.clone();
        let mut rng = rand::thread_rng();
        let n = ind.cells.len();
        let between = Uniform::from(0..n);
        for _ in 0..n_mutations {
            let row = between.sample(&mut rng);
            let col = between.sample(&mut rng);
            match ind.cells[row][col] {
                LCell::Alive => ind.cells[row][col] = LCell::Dead,
                LCell::Dead => ind.cells[row][col] = LCell::Alive,
            };
        }
        ind
    }
}
