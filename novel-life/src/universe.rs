use crate::lcell::LCell;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Universe {
    cells: Vec<Vec<LCell>>,
}

impl Universe {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![vec![LCell::Dead; size]; size],
        }
    }
}

impl From<&str> for Universe {
    fn from(s: &str) -> Self {
        let cells = s
            .lines()
            .map(|l| l.chars().map(LCell::from).collect::<Vec<LCell>>())
            .collect::<Vec<Vec<LCell>>>();
        let size = cells.len();
        for v in cells.iter() {
            assert!(v.len() == size);
        }
        Universe { cells }
    }
}

impl From<&Vec<Vec<LCell>>> for Universe {
    fn from(cells: &Vec<Vec<LCell>>) -> Self {
        Universe{ cells: cells.clone() }
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('┌');
        for _ in 0..self.cells.len() {
            s.push('─');
        }
        s.push('┐');
        s.push('\n');
        for v in self.cells.iter() {
            s.push('│');
            s.push_str(&v.iter().map(|c| c.to_string()).collect::<String>());
            s.push('│');
            s.push('\n');
        }
        s.push('└');
        for _ in 0..self.cells.len() {
            s.push('─');
        }
        s.push('┘');
        s.push('\n');
        write!(f, "{}", s)
    }
}

pub fn compute_distance(u0: &Universe, u1: &Universe) -> usize {
    u0.cells
        .iter()
        .zip(u1.cells.iter())
        .map(|(l0, l1)| {
            l0.iter()
                .zip(l1.iter())
                .map(|(c0, c1)| if c0 == c1 { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn count_alive_cells(u: &Universe) -> usize {
    u.cells
        .iter()
        .map(|l| {
            l.iter()
                .map(|&c| if c == LCell::Alive { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn seed(u: &Universe, seed: &Vec<Vec<LCell>>) -> Universe {
    let mut u = u.clone();
    let offset = u.cells.len() / 2 - seed.len() / 2;
    for row in 0..seed.len() {
        for col in 0..seed[0].len() {
            u.cells[offset + row][offset + col] = seed[row][col];
        }
    }
    u
}

pub fn simulate(u: &Universe, k: usize) -> Universe {
    let mut u = u.clone();
    for _ in 0..k {
        let u_new = tick(&u);
        if u_new == u {
            return u_new;
        }
        u = u_new;
    }
    u
}

pub fn simulate_with_history(u: &Universe, k: usize) -> Vec<Universe> {
    let mut u = u.clone();
    let mut history = Vec::new();
    history.push(u.clone());
    for _ in 0..k {
        let u_new = tick(&u);
        if u_new == u {
            return history;
        }
        u = u_new;
        history.push(u.clone())
    }
    history
}

pub fn measure_symmetry(u: &Universe) -> usize {
    if u.cells.len() % 2 == 0 {
        let n2 = u.cells.len() / 2;
        u.cells
            .iter()
            .map(|r| {
                r.iter()
                    .take(n2)
                    .zip(r.iter().rev())
                    .map(|(c0, c1)| {
                        if *c0 == LCell::Alive && c0 == c1 {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    } else {
        todo!();
    }
}

pub fn tick(u: &Universe) -> Universe {
    let mut new_u = u.clone();
    let size = u.cells.len() as i32;
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let mut n_alive = 0;
            for delta_y in [-1, 0, 1] {
                for delta_x in [-1, 0, 1] {
                    if delta_y == 0 && delta_x == 0 {
                        continue;
                    }
                    let y = i + delta_y;
                    let x = j + delta_x;
                    if u.cells[y as usize][x as usize] == LCell::Alive {
                        n_alive += 1;
                    }
                }
            }
            if n_alive < 2 {
                new_u.cells[i as usize][j as usize] = LCell::Dead;
            } else if n_alive == 2 {
                // keep cell in its current state
            } else if n_alive == 3 {
                new_u.cells[i as usize][j as usize] = LCell::Alive;
            } else {
                new_u.cells[i as usize][j as usize] = LCell::Dead;
            }
        }
    }
    new_u
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_patterns() {
        let u = Universe::from(
            ".....
.....
.###.
.....
.....",
        );
        let u_expected = Universe::from(
            ".....
..#..
..#..
..#..
.....",
        );
        let u = tick(&u);
        assert_eq!(u, u_expected);

        let u = Universe::from(
            "..........
..........
..........
..........
...###....
......#...
...#......
....#.#...
....#..#..
..........",
        );
        let u_expected = Universe::from(
            "..........
..........
..........
..........
..........
..........
..........
..........
..........
..........",
        );
        let u = simulate(&u, 10);
        assert_eq!(u, u_expected);
    }

    #[test]
    fn test_symmetry() {
        let u = Universe::from(
            "......
......
##..##
..##..
......
......",
        );
        assert_eq!(measure_symmetry(&u), 3);
    }

    #[test]
    fn test_distance() {
        let u0 = Universe::from(
            "......
......
##..##
..##..
......
......",
        );
        let u1 = Universe::from(
            "......
......
##....
..##..
##....
......",
        );
        assert_eq!(compute_distance(&u0, &u1), 4);
    }
}
