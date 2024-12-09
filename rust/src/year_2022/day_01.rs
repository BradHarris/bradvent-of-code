use crate::solver::Solver;

#[derive(Default)]
pub struct Solution {
    calorie_counts: Vec<u32>,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        let mut calorie_counts =
            input
                .lines()
                .map(|l| l.parse::<u32>().ok())
                .fold(vec![0], |mut acc, n| {
                    if let Some(n) = n {
                        let total_calories = acc.last_mut().unwrap();
                        *total_calories += n;
                    } else {
                        acc.push(0);
                    }
                    acc
                });

        calorie_counts.sort();
        calorie_counts.reverse();
        self.calorie_counts = calorie_counts;
    }

    fn solve_part1(&self) -> String {
        self.calorie_counts.first().unwrap().to_string()
    }

    fn solve_part2(&self) -> String {
        self.calorie_counts[0..3].iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "24000");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "45000");
    }
}

const INPUT: &str = include_str!("../../../inputs/2022/day_01.txt");
