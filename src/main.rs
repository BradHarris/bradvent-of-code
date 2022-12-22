use solver::Solvers;

mod day_template;
mod solver;
mod utils;
mod year_2020;
mod year_2021;
mod year_2022;

struct Args {
    day: usize,
    year: usize,
    runs: usize,
}

fn main() {
    let mut solvers = Solvers::new();
    let mut args = Args {
        day: 0, // 0 implies run all
        year: 2022,
        runs: 1,
    };

    let raw_args: Vec<String> = std::env::args().collect();

    if let Some(runs) = raw_args.get(2).and_then(|r| r.parse::<usize>().ok()) {
        args.runs = runs;
    }

    if let Some(year) = raw_args.get(3).and_then(|r| r.parse::<usize>().ok()) {
        args.year = year;
    }

    if let Some(day) = raw_args.get(1).and_then(|r| r.parse::<usize>().ok()) {
        args.day = day;
    }

    if args.day == 0 {
        Solvers::run_all(args.year, args.runs);
    } else {
        Solvers::run(args.year, args.day, args.runs);
    };
}
