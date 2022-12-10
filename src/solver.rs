pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
// pub mod day10;
// pub mod day11;
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
    fn parse(&mut self, input: &[String]);
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub fn get_solvers() -> Vec<Box<dyn Solver>> {
    let solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(day1::Day1::new()),
        Box::new(day2::Day2::new()),
        Box::new(day3::Day3::new()),
        Box::new(day4::Day4::new()),
        Box::new(day5::Day5::new()),
        Box::new(day6::Day6::new()),
        Box::new(day7::Day7::new()),
        Box::new(day8::Day8::new()),
        Box::new(day9::Day9::default()),
        // Box::new(day10::Day10::new()),
        // Box::new(day11::Day11::new()),
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
    ];

    solvers
}
