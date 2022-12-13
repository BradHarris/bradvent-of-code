use solver::Solvers;

mod day_template;
mod solver;
mod utils;
mod year_2020;
mod year_2021;
mod year_2022;

fn main() {
    let mut solvers = Solvers::new();
    let args: Vec<String> = std::env::args().collect();
    if let Some(a) = args.get(1) {
        let day = a.parse::<usize>().unwrap();
        solvers.run(day - 1);
    } else {
        solvers.run_all();
    };
}
