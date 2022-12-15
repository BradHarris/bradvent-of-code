use std::time::Instant;

use crate::year_2022::*;

pub trait Solver {
    fn get_input(&self) -> &'static str;
    fn with_input(&mut self, input: &str);
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
                Box::new(day_01::Solution::default()),
                Box::new(day_02::Solution::default()),
                Box::new(day_03::Solution::default()),
                Box::new(day_04::Solution::default()),
                Box::new(day_05::Solution::default()),
                Box::new(day_06::Solution::default()),
                Box::new(day_07::Solution::default()),
                Box::new(day_08::Solution::default()),
                Box::new(day_09::Solution::default()),
                Box::new(day_10::Solution::default()),
                Box::new(day_11::Solution::default()),
                Box::new(day_12::Solution::default()),
                Box::new(day_13::Solution::default()),
                Box::new(day_14::Solution::default()),
                Box::new(day_15::Solution::default()),
                // Box::new(day_16::Solution::default()),
                // Box::new(day_17::Solution::default()),
                // Box::new(day_18::Solution::default()),
                // Box::new(day_19::Solution::default()),
                // Box::new(day_20::Solution::default()),
                // Box::new(day_21::Solution::default()),
                // Box::new(day_22::Solution::default()),
                // Box::new(day_23::Solution::default()),
                // Box::new(day_24::Solution::default()),
                // Box::new(day_25::Solution::default()),
            ],
        }
    }

    pub fn run_all(&mut self) {
        for day in 0..self.solvers.len() {
            self.run(day);
        }
    }

    pub fn run(&mut self, day: usize) {
        let d = day + 1;
        println!("\n--- DAY {d:0>2} ---");

        let solver = self.solvers.get_mut(day).unwrap();

        #[cfg(debug_assertions)]
        let run_count = 1;
        #[cfg(not(debug_assertions))]
        let run_count = 10;

        println!("averaged over {run_count} runs");
        let part1_start = Instant::now();
        let mut solution1 = "".into();
        for _ in 0..run_count {
            solver.with_input(solver.get_input());
            solution1 = solver.solve_part1();
        }
        let part1_dur = part1_start.elapsed() / run_count;

        let part2_start = Instant::now();
        let mut solution2 = "".into();
        for _ in 0..run_count {
            solver.with_input(solver.get_input());
            solution2 = solver.solve_part2();
        }
        let part2_dur = part2_start.elapsed() / run_count;

        println!("part 1 ({part1_dur:?}): {solution1}");
        println!("part 2 ({part2_dur:?}): {solution2}");
    }
}
