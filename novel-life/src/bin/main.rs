// use std::io::Write;
// use crossterm::{cursor, ExecutableCommand};

use novel_life::{individual::Individual, universe::Universe};

fn main() {
    let mut archive = Vec::new();
    let mut parents = vec![Individual::new(5); 4];
    for _ in 0..10 {
        let mut offspring = parents.clone();
        for i in 0..offspring.len() {
            offspring[i] = offspring[i].mutate(1);

            let universe = Universe::new(30);
            let universe = universe.seed(&offspring[i].cells, 12, 12);
            let universe = simulate(&universe, 100);

            let mut distances = archive.iter().map(|u: &Universe| u.distance(&universe)).collect::<Vec<u32>>();
            distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            offspring[i].novelty = distances[..4].iter().sum::<u32>();

            archive.push(universe);
        }
        let mut population = parents.clone();
        population.append(&mut offspring);
        population.sort_unstable_by_key(|a| a.novelty);

        parents = population[..4].to_vec();
    }
}

fn simulate(u: &Universe, k: usize) -> Universe {
    let mut u = u.clone();
    for _ in 0..k {
        u = u.tick();
    }
    u
}

// fn simulate_with_history(u: &Universe, k: usize) -> Vec<Universe> {
//     let mut history = Vec::new();
//     let mut u = u.clone();
//     history.push(u.clone());
//     for _ in 0..k {
//         u = u.tick();
//         history.push(u.clone())
//     }
//     history
// }

// fn visualize(history: Vec<Universe>) {
//     let mut stdout = std::io::stdout();
//     for u in history {
//         let s = u.to_string();
//         println!("{:?}", s);
//         // for v in u.cells {
//         //     writeln!(stdout, "{}", v.iter().map(|c| c.to_string()).collect::<String>()).unwrap();
//         // }
//         // std::thread::sleep(std::time::Duration::from_millis(200));
//         // if !first_line {
//         //     stdout.execute(cursor::MoveUp(n_rows as u16)).unwrap();
//         // } else {
//         //     first_line = false;
//         // }
//     }
// }
