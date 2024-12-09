use crate::solver::Solver;

#[derive(Default, Debug)]
pub struct Solution {
    input: Vec<String>,
}

fn add_first_and_last(input: &String) -> u32 {
    let numbers = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    (numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0)).to_owned()
}

const DIGITS: &'static [&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.input = input.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
    }

    fn solve_part1(&self) -> String {
        self.input
            .iter()
            .map(add_first_and_last)
            .sum::<u32>()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.input
            .iter()
            .map(|l| {
                let l = l
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
                    .replace("zero", "z0o");

                add_first_and_last(&l)
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }

    fn get_example_input2<'a>() -> &'a str {
        "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "142");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input2());
        let solution = solver.solve_part2();
        assert_eq!(solution, "281");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "54916");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "54728");
    }
}

const INPUT: &str = include_str!("../../../inputs/2023/day_01.txt");
