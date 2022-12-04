use std::str::FromStr;

use super::Solver;

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

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {}
    }

    fn parse(&self, input: &Vec<String>) -> Vec<(Section, Section)> {
        input
            .iter()
            .map(|l| {
                let (first, second) = l.split_once(',').unwrap();
                (
                    first.parse::<Section>().unwrap(),
                    second.parse::<Section>().unwrap(),
                )
            })
            .collect()
    }
}

impl Solver for Day4 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        self.parse(input)
            .iter()
            .filter(|(first, second)| {
                first.is_contained_by(second) || second.is_contained_by(first)
            })
            .count()
    }
    fn solve_part2(&self, input: &Vec<String>) -> usize {
        self.parse(input)
            .iter()
            .filter(|(first, second)| {
                first.is_overlapped_by(second) || second.is_contained_by(first)
            })
            .count()
    }
}

#[cfg(test)]
mod test {}
