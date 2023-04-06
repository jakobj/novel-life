use std::io::Write;
use crossterm::{cursor, ExecutableCommand};

fn main() {
//     let s = "........
// .##.....
// .##.....
// ...##...
// ...##...
// ........";
    // let universe = parse(s);

    let mut universe = Universe{ cells: vec![vec![LCell{ state: State::Dead }; 30]; 30] };

    universe.cells[15][15].state = State::Alive;
    universe.cells[15][14].state = State::Alive;
    universe.cells[15][16].state = State::Alive;

    let history = simulate_with_history(&universe, 10);
    visualize(history);
}

fn parse(s: &str) -> Universe {
    let cells = s.lines().map(|l| l.chars().map(|c| LCell{ state: State::from(c) }).collect::<Vec<LCell>>()).collect::<Vec<Vec<LCell>>>();
    let n_cols = cells[0].len();
    for v in cells.iter() {
        assert!(v.len() == n_cols);
    }
    Universe{ cells }
}

fn step(u: &Universe) -> Universe {
    let mut universe = u.clone();
    let n_rows = u.cells.len() as i32;
    let n_cols = u.cells[0].len() as i32;
    for row in 0..n_rows {
        for col in 0..n_cols {
            let mut n_alive = 0;
            for delta_y in [-1, 0, 1] {
                for delta_x in [-1, 0, 1] {
                    if delta_y == 0 && delta_x == 0 {
                        continue;
                    }
                    let y = row + delta_y;
                    let x = col + delta_x;
                    if (y >= 0 && y < n_rows) && (x >= 0 && x < n_cols) {
                        if u.cells[y as usize][x as usize].state == State::Alive {
                            n_alive += 1;
                        }
                    }
                }
            }
            if n_alive < 2 {
                universe.cells[row as usize][col as usize].state = State::Dead;
            } else if n_alive == 2 {
                // keep cell in its current state
            } else if n_alive == 3 {
                universe.cells[row as usize][col as usize].state = State::Alive;
            } else {
                universe.cells[row as usize][col as usize].state = State::Dead;
            }
        }
    }
    universe
}

fn simulate(u: &Universe, k: usize) -> Universe {
    let mut u = u.to_owned();
    for _ in 0..k {
        u = step(&u);
    }
    u
}

fn simulate_with_history(u: &Universe, k: usize) -> Vec<Universe> {
    let mut history = Vec::new();
    let mut u = u.to_owned();
    history.push(u.clone());
    for _ in 0..k {
        u = step(&u);
        history.push(u.clone())
    }
    history
}

fn visualize(history: Vec<Universe>) {
    let mut stdout = std::io::stdout();
    let n_rows = history[0].cells.len();
    let mut first_line = true;
    for u in history {
        if !first_line {
            stdout.execute(cursor::MoveUp(n_rows as u16)).unwrap();
        } else {
            first_line = false;
        }
        for v in u.cells {
            writeln!(stdout, "{}", v.iter().map(|c| c.state.to_string()).collect::<String>()).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

#[derive(Clone,Debug)]
struct Universe {
    cells: Vec<Vec<LCell>>,
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

#[derive(Clone,Copy,Debug)]
struct LCell {
    state: State,
}

impl std::fmt::Display for LCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum State {
    Alive,
    Dead,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '#' => State::Alive,
            '.' => State::Dead,
            _ => panic!("unexpected cell state string {}", c),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Alive => write!(f, "#"),
            State::Dead => write!(f, "."),
        }
    }
}
