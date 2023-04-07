use crate::lcell::LCell;

#[derive(Clone,Debug)]
pub struct Universe {
    cells: Vec<Vec<LCell>>,
}

impl Universe {
    pub fn new(n: usize) -> Self {
        Self{ cells: vec![vec![LCell::Dead; n]; n] }
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

    pub fn seed(&self, seed: &Vec<Vec<LCell>>, row_offset: usize, col_offset: usize) -> Self {
        let mut u = self.clone();
        for row in 0..seed.len() {
            for col in 0..seed[0].len() {
                u.cells[row_offset + row][col_offset + col] = seed[row][col];
            }
        }
        u
    }

    pub fn tick(&self) -> Self {
        let mut u = self.clone();
        let n = u.cells.len() as i32;
        for row in 0..n {
            for col in 0..n {
                let mut n_alive = 0;
                for delta_y in [-1, 0, 1] {
                    for delta_x in [-1, 0, 1] {
                        if delta_y == 0 && delta_x == 0 {
                            continue;
                        }
                        let y = row + delta_y;
                        let x = col + delta_x;
                        if (y >= 0 && y < n) && (x >= 0 && x < n) {
                            if u.cells[y as usize][x as usize] == LCell::Alive {
                                n_alive += 1;
                            }
                        }
                    }
                }
                if n_alive < 2 {
                    u.cells[row as usize][col as usize] = LCell::Dead;
                } else if n_alive == 2 {
                    // keep cell in its current state
                } else if n_alive == 3 {
                    u.cells[row as usize][col as usize] = LCell::Alive;
                } else {
                    u.cells[row as usize][col as usize] = LCell::Dead;
                }
            }
        }
        u
    }

    // fn from_map_str(s: &str) -> Self {
    //     let cells = s.lines().map(|l| l.chars().map(|c| LCell::from(c) ).collect::<Vec<LCell>>()).collect::<Vec<Vec<LCell>>>();
    //     let n_cols = cells[0].len();
    //     for v in cells.iter() {
    //         assert!(v.len() == n_cols);
    //     }
    //     Universe{ cells }
    // }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for v in self.cells.iter() {
            s.push_str(&v.iter().map(|c| c.to_string()).collect::<String>());
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}
