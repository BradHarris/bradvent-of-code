use solver::Solvers;

mod solver;
mod utils;

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
