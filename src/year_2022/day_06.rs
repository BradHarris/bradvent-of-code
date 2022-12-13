use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Default)]
pub struct Solution {
    input: String,
}

impl Solution {
    fn start_of_distinct_chars(&self, num_distinct: usize) -> usize {
        let chars = self.input.chars().collect::<Vec<char>>();
        let chars = chars.windows(num_distinct);

        let mut counter = num_distinct;
        for c in chars {
            let unique_chars = c.iter().collect::<HashSet<&char>>().len();
            if unique_chars == num_distinct {
                break;
            }

            counter += 1;
        }

        counter
    }
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input.to_owned();
    }

    fn solve_part1(&self) -> String {
        self.start_of_distinct_chars(4).to_string()
    }

    fn solve_part2(&self) -> String {
        self.start_of_distinct_chars(14).to_string()
    }
}

#[cfg(test)]
mod test {}
