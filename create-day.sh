#!/bin/bash

touch ./src/input/day$1.txt

echo "use super::Solver;

pub struct Day$1 {
    input: Vec<String>
}

impl Day$1 {
    pub fn new() -> Day$1 {
        Day$1 { input: Vec::new() }
    }
}

impl Solver for Day$1 {
    fn parse(&mut self, input: &[String]) {
        self.input = input.to_owned();
    }

    fn solve_part1(&self) -> String {
        \"not implemented!\".to_string()
    }

    fn solve_part2(&self) -> String {
        \"not implemented!\".to_string()
    }
}

#[cfg(test)]
mod test {}" > ./src/solver/day$1.rs