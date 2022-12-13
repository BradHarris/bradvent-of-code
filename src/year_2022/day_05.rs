use crate::solver::Solver;

#[derive(Default)]
pub struct Solution {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        for line in input.split('\n') {
            if line.is_empty() {
                break;
            }
            line.chars()
                .enumerate()
                .skip(1)
                .step_by(4)
                .for_each(|(i, c)| {
                    let stack_index = (i - 1) / 4;
                    while stack_index >= self.stacks.len() {
                        self.stacks.push(Vec::new());
                    }

                    if c.is_alphabetic() {
                        let stack = self.stacks.get_mut(stack_index).unwrap();
                        stack.insert(0, c);
                    }
                });
        }

        for line in input.split('\n') {
            if !line.starts_with("move") {
                continue;
            }
            let line = line.strip_prefix("move ").unwrap();
            let (amount, rest) = line.split_once(" from ").unwrap();
            let (from, to) = rest.split_once(" to ").unwrap();
            let amount = amount.parse::<usize>().unwrap();
            let from = from.parse::<usize>().unwrap() - 1;
            let to = to.parse::<usize>().unwrap() - 1;
            self.moves.push((amount, from, to));
        }
    }

    fn solve_part1(&self) -> String {
        let mut stacks = self.stacks.clone();

        self.moves.iter().for_each(|(amount, from, to)| {
            let stack_from = stacks.get_mut(*from).unwrap();
            let mut temp = Vec::new();
            for _i in 0..*amount {
                let c = stack_from.pop().unwrap();
                temp.push(c);
            }
            let stack_to = stacks.get_mut(*to).unwrap();
            stack_to.append(&mut temp);
        });

        stacks
            .iter()
            .map(|s| *s.last().unwrap())
            .collect::<String>()
    }

    fn solve_part2(&self) -> String {
        let mut stacks = self.stacks.clone();

        self.moves.iter().for_each(|(amount, from, to)| {
            let stack_from = stacks.get(*from).unwrap();
            let new_from = stack_from[0..stack_from.len() - amount].to_vec();
            let mut new_to = stack_from[stack_from.len() - amount..].to_vec();

            stacks[*from] = new_from;
            stacks.get_mut(*to).unwrap().append(&mut new_to);
        });

        stacks
            .iter()
            .map(|s| *s.last().unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {}
