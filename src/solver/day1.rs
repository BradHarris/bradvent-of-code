use super::Solver;

pub struct Day1 {
    calorie_counts: Vec<u32>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            calorie_counts: Vec::new(),
        }
    }
}

impl Solver for Day1 {
    fn parse(&mut self, input: &[String]) {
        let mut calorie_counts =
            input
                .iter()
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
mod test {}
