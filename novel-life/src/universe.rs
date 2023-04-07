use crate::lcell::LCell;

#[derive(Clone, Debug)]
pub struct Universe {
    cells: Vec<Vec<LCell>>,
}

impl Universe {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![vec![LCell::Dead; size]; size],
        }
    }

    pub fn distance(&self, other: &Self) -> u32 {
        let mut sum = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                sum += match (self.cells[i][j], other.cells[i][j]) {
                    (LCell::Alive, LCell::Alive) => 0,
                    (LCell::Alive, LCell::Dead) => 1,
                    (LCell::Dead, LCell::Alive) => 1,
                    (LCell::Dead, LCell::Dead) => 0,
                };
            }
        }
        sum
    }

    pub fn n_alive(&self) -> usize {
        self.cells
            .iter()
            .map(|l| {
                l.iter()
                    .map(|&c| if c == LCell::Alive { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn seed(&self, seed: &Vec<Vec<LCell>>, row_offset: usize, col_offset: usize) -> Self {
        let mut u = self.clone();
        for row in 0..seed.len() {
            for col in 0..seed[0].len() {
                u.cells[row_offset + row][col_offset + col] = seed[row][col];
            }
        }
        u
    }

    pub fn simulate(&self, k: usize) -> Self {
        let mut u = self.clone();
        for _ in 0..k {
            u = u.tick();
        }
        u
    }

    pub fn simulate_with_history(&self, k: usize) -> Vec<Self> {
        let mut history = Vec::new();
        let mut u = self.clone();
        history.push(u.clone());
        for _ in 0..k {
            u = u.tick();
            history.push(u.clone())
        }
        history
    }

    pub fn symmetry(&self) -> u32 {
        if self.cells.len() % 2 == 0 {
            let mut s = 0;
            let n = self.cells.len();
            let n2 = n / 2;
            for i in 0..self.cells.len() {
                for j in 0..n2 {
                    if self.cells[i][j] == LCell::Alive
                        && self.cells[i][j] == self.cells[i][(n as i32 - 1 - j as i32) as usize] {
                        s += 1;
                    }
                }
            }
            s
        } else {
            todo!();
        }
    }

    pub fn tick(&self) -> Self {
        let mut u = self.clone();
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
                        if self.cells[y as usize][x as usize] == LCell::Alive {
                            n_alive += 1;
                        }
                    }
                }
                if n_alive < 2 {
                    u.cells[i as usize][j as usize] = LCell::Dead;
                } else if n_alive == 2 {
                    // keep cell in its current state
                } else if n_alive == 3 {
                    u.cells[i as usize][j as usize] = LCell::Alive;
                } else {
                    u.cells[i as usize][j as usize] = LCell::Dead;
                }
            }
        }
        u
    }
}

impl From<&str> for Universe {
    fn from(s: &str) -> Self {
        let cells = s
            .lines()
            .map(|l| l.chars().map(|c| LCell::from(c)).collect::<Vec<LCell>>())
            .collect::<Vec<Vec<LCell>>>();
        let size = cells.len();
        for v in cells.iter() {
            assert!(v.len() == size);
        }
        Universe { cells }
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

impl Eq for Universe {}

impl PartialEq for Universe {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
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
        let u = u.tick();
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
        let u = u.simulate(10);
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
        assert_eq!(u.symmetry(), 3);
    }
}
