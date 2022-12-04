use super::Solver;

#[derive(Debug)]
struct Elf {
    total_calories: usize,
}

impl Elf {
    fn new() -> Elf {
        Elf { total_calories: 0 }
    }
}

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {}
    }

    fn parse(&self, input: &Vec<String>) -> Vec<Elf> {
        let mut elves =
            input
                .iter()
                .map(|l| l.parse::<usize>().ok())
                .fold(vec![Elf::new()], |mut acc, n| {
                    if let Some(n) = n {
                        let elf = acc.last_mut().unwrap();
                        elf.total_calories += n;
                    } else {
                        acc.push(Elf::new());
                    }
                    acc
                });

        elves.sort_by_key(|e| e.total_calories);
        elves.reverse();
        elves
    }
}

impl Solver for Day1 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        let elves = self.parse(input);

        elves.first().unwrap().total_calories
    }
    fn solve_part2(&self, input: &Vec<String>) -> usize {
        let elves = self.parse(input);

        elves[0..3].iter().map(|e| e.total_calories).sum()
    }
}

#[cfg(test)]
mod test {}
