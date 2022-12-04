#!/bin/bash

touch ./src/input/day$1.txt

echo "use super::Solver;

pub struct Day$1 {}

impl Day$1 {
    pub fn new() -> Day$1 { Day$1 {  } }
}

impl Solver for Day$1 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        todo!()
    }
    fn solve_part2(&self, input: &Vec<String>) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test {}" > ./src/solver/day$1.rs