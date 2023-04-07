// use std::io::Write;
// use crossterm::{cursor, ExecutableCommand};

use novel_life::{individual::Individual, universe::Universe};

fn main() {
    let mut archive = Vec::new();
    let mut parents = vec![Individual::new(5); 4];
    for _ in 0..50 {
        let mut offspring = parents.clone();
        for i in 0..offspring.len() {
            offspring[i] = offspring[i].mutate(1);

            let universe = Universe::new(100);
            let universe = universe.seed(&offspring[i].cells, 48, 48);
            let universe = universe.simulate(500);
            if universe.n_alive() > 100 {
                println!("{}", universe.n_alive());
                println!("{:?}", offspring[i].cells);
                println!("{}", universe);
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
