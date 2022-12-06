use std::collections::HashSet;

use super::Solver;

pub struct Day6 {
    input: Vec<String>,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 { input: Vec::new() }
    }

    fn start_of_distinct_chars(&self, num_distinct: usize) -> usize {
        let input = self.input.first().unwrap();

        let mut marker: Vec<char> = Vec::with_capacity(4);

        let mut chars = input.chars();
        let mut counter = 0;
        while let Some(c) = chars.next() {
            if marker.len() == num_distinct {
                let unique_chars = marker
                    .iter()
                    .map(|c| c.clone())
                    .collect::<HashSet<char>>()
                    .len();
                if unique_chars == num_distinct {
                    break;
                }

                marker.remove(0);
            }
            marker.push(c);
            counter += 1;
        }

        counter
    }
}

impl Solver for Day6 {
    fn parse(&mut self, input: &[String]) {
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
