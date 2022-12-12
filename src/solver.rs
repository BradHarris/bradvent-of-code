use std::time::Instant;

use crate::utils::read_input;

pub mod day_template;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub trait Solver {
    fn parse(&mut self, input: &str);
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub struct Solvers {
    solvers: Vec<Box<dyn Solver>>,
}

impl Solvers {
    pub fn new() -> Self {
        Self {
            solvers: vec![
                Box::new(day1::Solution::default()),
                Box::new(day2::Solution::default()),
                Box::new(day3::Solution::default()),
                Box::new(day4::Solution::default()),
                Box::new(day5::Solution::default()),
                Box::new(day6::Solution::default()),
                Box::new(day7::Solution::default()),
                Box::new(day8::Solution::default()),
                Box::new(day9::Solution::default()),
                Box::new(day10::Solution::default()),
                Box::new(day11::Solution::default()),
                // Box::new(day12::Day12::new()),
                // Box::new(day13::Day13::new()),
                // Box::new(day14::Day14::new()),
                // Box::new(day15::Day15::new()),
                // Box::new(day16::Day16::new()),
                // Box::new(day17::Day17::new()),
                // Box::new(day18::Day18::new()),
                // Box::new(day19::Day19::new()),
                // Box::new(day20::Day20::new()),
                // Box::new(day21::Day21::new()),
                // Box::new(day22::Day22::new()),
                // Box::new(day23::Day23::new()),
                // Box::new(day24::Day24::new()),
                // Box::new(day25::Day25::new()),
            ],
        }
    }

    pub fn run_all(&mut self) {
        for day in 0..self.solvers.len() {
            self.run(day);
        }
    }

    pub fn run(&mut self, day: usize) {
        println!("\n--- DAY {} ---", day + 1);
        let input = read_input(&format!("./src/input/day{}.txt", day + 1));

        let solver = self.solvers.get_mut(day).unwrap();

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
