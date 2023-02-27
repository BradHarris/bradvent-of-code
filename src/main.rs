use utils::print_time_results;

mod day_template;
mod solver;
mod utils;
mod year_2018;
mod year_2020;
mod year_2021;
mod year_2022;

#[derive(Debug)]
struct Args {
    day: usize,
    year: usize,
    runs: usize,
}

fn main() {
    let mut args = Args {
        day: 0, // 0 implies run all
        year: 2022,
        runs: 1,
    };

    let raw_args: Vec<String> = std::env::args().collect();

    if let Some(day) = raw_args.get(1).and_then(|r| r.parse::<usize>().ok()) {
        args.day = day;
    }

    if let Some(runs) = raw_args.get(2).and_then(|r| r.parse::<usize>().ok()) {
        args.runs = runs;
    }

    if let Some(year) = raw_args.get(3).and_then(|r| r.parse::<usize>().ok()) {
        args.year = year;
    }

    println!("{args:?}");
    if args.day == 0 {
        let results = solver::run_all(args.year, args.runs);
        print_time_results(results, args.runs)
    } else {
        let result = solver::run(args.year, args.day, args.runs);
        if let Some(result) = result {
            print_time_results(vec![result], args.runs)
        }
    };
}
