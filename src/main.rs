use solver::Solvers;

mod day_template;
mod solver;
mod year_2020;
mod year_2021;
mod year_2022;

fn main() {
    let mut solvers = Solvers::new();
    let args: Vec<String> = std::env::args().collect();
    let runs = if let Some(r) = args.get(2).and_then(|r| r.parse::<usize>().ok()) {
        r
    } else {
        1
    };

    let year = if let Some(r) = args.get(3).and_then(|r| r.parse::<usize>().ok()) {
        r
    } else {
        2022
    };

    if let Some(a) = args.get(1) {
        let day = a.parse::<usize>().unwrap();
        solvers.run(year, day - 1, runs);
    } else {
        solvers.run_all(year, runs);
    };
}
