use std::time::Instant;

use crate::{solver::Solver, utils::read_input};

mod solver;
mod utils;

fn main() {
    let mut solvers: Vec<Box<dyn Solver>> = solver::get_solvers();

    let args: Vec<String> = std::env::args().collect();
    let days_to_solve = if let Some(a) = args.get(1) {
        let a = a.parse::<usize>().unwrap();
        a - 1..a
    } else {
        0..solvers.len()
    };

    for d in days_to_solve {
        println!("\n--- DAY {} ---", d + 1);
        let input = read_input(&format!("./src/input/day{}.txt", d + 1));

        let solver = solvers.get_mut(d).unwrap();

        let parser_start = Instant::now();
        solver.parse(&input);
        let parser_dur = parser_start.elapsed();

        let part1_start = Instant::now();
        println!("part 1: {}", solver.solve_part1());
        let part1_dur = part1_start.elapsed();

        let part2_start = Instant::now();
        println!("part 2: {}", solver.solve_part2());
        let part2_dur = part2_start.elapsed();

        println!(
            "parse: {:?}, part1: {:?}, part2: {:?} total: {:?}\n",
            parser_dur,
            part1_dur,
            part2_dur,
            parser_start.elapsed()
        );
    }
}
