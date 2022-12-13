use std::str::FromStr;

use crate::solver::Solver;

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn is_contained_by(&self, section: &Section) -> bool {
        self.start >= section.start && self.end <= section.end
    }

    fn is_overlapped_by(&self, section: &Section) -> bool {
        (self.start >= section.start && self.start <= section.end)
            || (self.end >= section.start && self.end <= section.end)
    }
}

impl FromStr for Section {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Section {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap(),
        })
    }
}

#[derive(Default)]
pub struct Solution {
    input: Vec<(Section, Section)>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input
            .split('\n')
            .map(|l| {
                let (first, second) = l.split_once(',').unwrap();
                (
                    first.parse::<Section>().unwrap(),
                    second.parse::<Section>().unwrap(),
                )
            })
            .collect();
    }

    fn solve_part1(&self) -> String {
        self.input
            .iter()
            .filter(|(first, second)| {
                first.is_contained_by(second) || second.is_contained_by(first)
            })
            .count()
            .to_string()
    }
    fn solve_part2(&self) -> String {
        self.input
            .iter()
            .filter(|(first, second)| {
                first.is_overlapped_by(second) || second.is_contained_by(first)
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {}
