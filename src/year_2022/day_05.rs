use crate::solver::Solver;

#[derive(Default)]
pub struct Solution {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
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

const INPUT: &'static str = "[P]     [C]         [M]
[D]     [P] [B]     [V] [S]
[Q] [V] [R] [V]     [G] [B]
[R] [W] [G] [J]     [T] [M]     [V]
[V] [Q] [Q] [F] [C] [N] [V]     [W]
[B] [Z] [Z] [H] [L] [P] [L] [J] [N]
[H] [D] [L] [D] [W] [R] [R] [P] [C]
[F] [L] [H] [R] [Z] [J] [J] [D] [D]
 1   2   3   4   5   6   7   8   9

move 4 from 9 to 1
move 6 from 3 to 1
move 7 from 4 to 1
move 2 from 8 to 5
move 1 from 9 to 7
move 1 from 8 to 5
move 3 from 6 to 4
move 6 from 1 to 5
move 14 from 1 to 2
move 1 from 6 to 1
move 2 from 6 to 2
move 9 from 5 to 9
move 2 from 4 to 5
move 2 from 5 to 3
move 6 from 9 to 6
move 4 from 1 to 2
move 2 from 1 to 2
move 5 from 6 to 1
move 1 from 4 to 9
move 4 from 9 to 4
move 2 from 3 to 7
move 2 from 4 to 9
move 2 from 9 to 6
move 5 from 2 to 9
move 1 from 4 to 9
move 1 from 4 to 3
move 5 from 9 to 8
move 1 from 6 to 5
move 3 from 7 to 5
move 2 from 1 to 6
move 5 from 6 to 8
move 1 from 9 to 4
move 1 from 6 to 5
move 9 from 2 to 7
move 1 from 2 to 3
move 1 from 4 to 6
move 8 from 5 to 4
move 1 from 6 to 1
move 2 from 8 to 6
move 1 from 6 to 4
move 7 from 4 to 6
move 1 from 3 to 1
move 1 from 3 to 4
move 3 from 4 to 1
move 2 from 3 to 4
move 2 from 4 to 5
move 3 from 5 to 7
move 7 from 8 to 2
move 5 from 1 to 2
move 12 from 7 to 6
move 2 from 1 to 9
move 2 from 9 to 1
move 1 from 7 to 5
move 6 from 2 to 3
move 5 from 2 to 6
move 6 from 2 to 6
move 4 from 3 to 1
move 3 from 2 to 1
move 1 from 5 to 4
move 7 from 1 to 2
move 1 from 4 to 8
move 7 from 2 to 9
move 5 from 2 to 8
move 2 from 6 to 8
move 21 from 6 to 9
move 8 from 9 to 1
move 2 from 6 to 1
move 3 from 8 to 7
move 6 from 6 to 4
move 7 from 1 to 8
move 1 from 9 to 1
move 7 from 7 to 3
move 1 from 7 to 4
move 1 from 7 to 4
move 7 from 8 to 1
move 5 from 4 to 8
move 10 from 1 to 2
move 3 from 1 to 4
move 3 from 2 to 9
move 1 from 4 to 5
move 3 from 3 to 6
move 1 from 6 to 4
move 1 from 6 to 7
move 1 from 7 to 8
move 7 from 2 to 4
move 10 from 9 to 1
move 10 from 4 to 5
move 2 from 5 to 2
move 2 from 2 to 1
move 11 from 8 to 9
move 7 from 1 to 4
move 1 from 6 to 1
move 1 from 8 to 3
move 1 from 4 to 6
move 6 from 4 to 5
move 1 from 5 to 7
move 1 from 6 to 8
move 6 from 1 to 6
move 19 from 9 to 2
move 1 from 1 to 8
move 1 from 4 to 7
move 9 from 2 to 6
move 1 from 9 to 2
move 2 from 8 to 1
move 1 from 1 to 9
move 7 from 3 to 6
move 3 from 9 to 2
move 5 from 2 to 6
move 1 from 9 to 3
move 15 from 6 to 7
move 6 from 6 to 7
move 1 from 1 to 9
move 5 from 6 to 2
move 1 from 6 to 1
move 6 from 5 to 8
move 1 from 3 to 4
move 1 from 9 to 7
move 6 from 8 to 1
move 3 from 4 to 6
move 1 from 6 to 1
move 3 from 5 to 2
move 1 from 5 to 7
move 5 from 1 to 5
move 2 from 6 to 9
move 2 from 9 to 2
move 7 from 5 to 1
move 1 from 5 to 7
move 1 from 5 to 9
move 20 from 7 to 1
move 23 from 1 to 7
move 1 from 1 to 2
move 4 from 7 to 9
move 4 from 9 to 8
move 1 from 9 to 2
move 16 from 7 to 6
move 4 from 1 to 5
move 9 from 7 to 6
move 11 from 2 to 6
move 1 from 1 to 9
move 1 from 1 to 7
move 1 from 8 to 2
move 1 from 9 to 7
move 4 from 5 to 2
move 3 from 8 to 3
move 2 from 2 to 4
move 2 from 7 to 4
move 4 from 4 to 9
move 28 from 6 to 9
move 5 from 2 to 7
move 8 from 6 to 5
move 6 from 2 to 6
move 2 from 7 to 3
move 5 from 5 to 7
move 1 from 5 to 9
move 14 from 9 to 4
move 18 from 9 to 8
move 5 from 6 to 4
move 6 from 7 to 8
move 1 from 2 to 6
move 19 from 4 to 7
move 1 from 2 to 5
move 1 from 9 to 3
move 2 from 5 to 2
move 14 from 7 to 3
move 1 from 5 to 3
move 12 from 8 to 6
move 6 from 6 to 5
move 4 from 5 to 4
move 21 from 3 to 4
move 10 from 8 to 3
move 2 from 3 to 2
move 7 from 4 to 6
move 2 from 8 to 1
move 2 from 2 to 3
move 5 from 7 to 2
move 2 from 1 to 4
move 3 from 3 to 7
move 2 from 5 to 7
move 2 from 2 to 7
move 2 from 2 to 3
move 7 from 4 to 1
move 3 from 1 to 4
move 3 from 2 to 5
move 2 from 1 to 5
move 7 from 4 to 3
move 15 from 6 to 2
move 1 from 1 to 4
move 1 from 5 to 1
move 14 from 3 to 1
move 9 from 4 to 1
move 5 from 7 to 1
move 1 from 3 to 5
move 1 from 4 to 2
move 20 from 1 to 2
move 17 from 2 to 5
move 1 from 3 to 7
move 5 from 7 to 3
move 6 from 5 to 1
move 3 from 3 to 2
move 10 from 1 to 9
move 3 from 5 to 6
move 12 from 5 to 6
move 1 from 5 to 1
move 15 from 6 to 5
move 13 from 5 to 3
move 1 from 5 to 1
move 10 from 3 to 2
move 3 from 3 to 2
move 1 from 5 to 3
move 2 from 3 to 6
move 1 from 3 to 4
move 2 from 6 to 4
move 3 from 4 to 2
move 8 from 9 to 4
move 8 from 4 to 8
move 7 from 2 to 1
move 5 from 8 to 7
move 2 from 2 to 3
move 13 from 1 to 2
move 2 from 3 to 8
move 2 from 9 to 7
move 3 from 8 to 1
move 2 from 1 to 2
move 2 from 8 to 4
move 6 from 7 to 2
move 3 from 1 to 8
move 1 from 7 to 5
move 24 from 2 to 1
move 2 from 8 to 5
move 15 from 1 to 4
move 1 from 5 to 8
move 9 from 1 to 4
move 2 from 8 to 5
move 26 from 2 to 4
move 1 from 5 to 8
move 1 from 5 to 8
move 50 from 4 to 1
move 1 from 8 to 9
move 1 from 4 to 6
move 1 from 4 to 9
move 22 from 1 to 5
move 1 from 6 to 2
move 1 from 5 to 8
move 1 from 2 to 4
move 1 from 8 to 1
move 28 from 1 to 3
move 2 from 9 to 4
move 21 from 5 to 8
move 1 from 1 to 8
move 1 from 5 to 8
move 1 from 5 to 7
move 3 from 4 to 8
move 1 from 7 to 9
move 1 from 9 to 7
move 20 from 8 to 4
move 2 from 8 to 1
move 1 from 7 to 6
move 2 from 1 to 4
move 27 from 3 to 1
move 4 from 8 to 4
move 1 from 6 to 9
move 19 from 4 to 2
move 5 from 2 to 5
move 1 from 4 to 1
move 1 from 9 to 2
move 17 from 1 to 9
move 1 from 3 to 8
move 15 from 9 to 2
move 2 from 4 to 8
move 2 from 5 to 8
move 2 from 5 to 9
move 3 from 9 to 8
move 9 from 1 to 2
move 2 from 1 to 3
move 4 from 4 to 5
move 2 from 5 to 7
move 1 from 8 to 5
move 2 from 3 to 8
move 4 from 5 to 2
move 1 from 9 to 6
move 5 from 8 to 5
move 1 from 7 to 9
move 29 from 2 to 3
move 1 from 8 to 6
move 1 from 9 to 7
move 2 from 2 to 8
move 2 from 5 to 2
move 2 from 7 to 5
move 4 from 5 to 9
move 1 from 5 to 9
move 10 from 3 to 4
move 10 from 4 to 7
move 1 from 3 to 4
move 5 from 2 to 9
move 5 from 8 to 6
move 1 from 6 to 5
move 2 from 6 to 3
move 4 from 6 to 7
move 1 from 5 to 2
move 2 from 2 to 7
move 5 from 7 to 8
move 8 from 7 to 2
move 6 from 8 to 7
move 14 from 2 to 5
move 3 from 7 to 3
move 1 from 4 to 7
move 2 from 7 to 2
move 3 from 2 to 8
move 3 from 8 to 5
move 8 from 9 to 1
move 3 from 7 to 2
move 2 from 7 to 4
move 17 from 3 to 6
move 8 from 1 to 6
move 16 from 5 to 2
move 1 from 5 to 2
move 1 from 3 to 1
move 21 from 6 to 7
move 1 from 4 to 8
move 7 from 7 to 8
move 1 from 1 to 3
move 11 from 7 to 2
move 7 from 2 to 6
move 8 from 8 to 5
move 2 from 7 to 4
move 4 from 5 to 6
move 8 from 2 to 8
move 17 from 2 to 3
move 4 from 5 to 3
move 7 from 6 to 9
move 2 from 6 to 9
move 1 from 4 to 1
move 1 from 4 to 2
move 3 from 6 to 2
move 1 from 6 to 8
move 1 from 4 to 1
move 1 from 7 to 5
move 10 from 9 to 2
move 1 from 5 to 6
move 1 from 8 to 2
move 1 from 1 to 4
move 12 from 3 to 4
move 1 from 6 to 2
move 2 from 8 to 6
move 1 from 1 to 2
move 1 from 9 to 8
move 2 from 8 to 7
move 6 from 3 to 2
move 1 from 3 to 5
move 8 from 4 to 9
move 22 from 2 to 9
move 7 from 3 to 5
move 3 from 8 to 2
move 2 from 7 to 8
move 3 from 6 to 9
move 1 from 2 to 9
move 1 from 6 to 2
move 4 from 8 to 5
move 5 from 5 to 9
move 1 from 3 to 6
move 1 from 5 to 6
move 2 from 4 to 1
move 2 from 2 to 4
move 4 from 4 to 6
move 1 from 1 to 5
move 5 from 6 to 3
move 35 from 9 to 1
move 4 from 9 to 1
move 1 from 4 to 7
move 3 from 3 to 7
move 37 from 1 to 7
move 2 from 2 to 3
move 3 from 3 to 7
move 1 from 5 to 8
move 2 from 1 to 8
move 2 from 5 to 2
move 1 from 6 to 9
move 16 from 7 to 1
move 5 from 1 to 5
move 3 from 8 to 2
move 10 from 7 to 9
move 6 from 7 to 9
move 3 from 2 to 1
move 4 from 5 to 3
move 2 from 1 to 2
move 5 from 7 to 9
move 5 from 7 to 9
move 5 from 5 to 3
move 8 from 3 to 7
move 6 from 9 to 4
move 8 from 7 to 3
move 2 from 3 to 6
move 1 from 6 to 7
move 1 from 6 to 7
move 5 from 4 to 9
move 3 from 7 to 1
move 2 from 2 to 8
move 1 from 8 to 6
move 6 from 1 to 8
move 1 from 7 to 9
move 1 from 3 to 9
move 4 from 3 to 2
move 8 from 1 to 6
move 1 from 3 to 9
move 5 from 8 to 4
move 2 from 3 to 1
move 1 from 8 to 2
move 4 from 9 to 1
move 2 from 1 to 5
move 1 from 8 to 5
move 11 from 9 to 5
move 1 from 2 to 8
move 10 from 5 to 4
move 1 from 1 to 9
move 3 from 5 to 4
move 5 from 2 to 3
move 1 from 5 to 1
move 9 from 9 to 4
move 1 from 6 to 7
move 1 from 3 to 9
move 4 from 3 to 1
move 1 from 2 to 4
move 1 from 1 to 4
move 1 from 4 to 7
move 5 from 1 to 3
move 1 from 3 to 2
move 1 from 8 to 3
move 3 from 9 to 5
move 1 from 2 to 9
move 4 from 1 to 4
move 1 from 7 to 4
move 2 from 5 to 8
move 1 from 7 to 6
move 4 from 3 to 1
move 1 from 5 to 8
move 1 from 3 to 4
move 22 from 4 to 1
move 11 from 1 to 9
move 2 from 1 to 4
move 11 from 1 to 6
move 8 from 6 to 7
move 1 from 8 to 7
move 7 from 9 to 2
move 6 from 7 to 6
move 2 from 4 to 9
move 2 from 7 to 1
move 14 from 6 to 3
move 2 from 3 to 1
move 3 from 6 to 7
move 6 from 1 to 3
move 8 from 9 to 6
move 7 from 4 to 6
move 7 from 6 to 8
move 1 from 9 to 1
move 2 from 9 to 8
move 4 from 3 to 4
move 1 from 8 to 4
move 1 from 4 to 3
move 6 from 3 to 7
move 7 from 2 to 5
move 8 from 4 to 6
move 1 from 7 to 2
move 1 from 5 to 7
move 6 from 7 to 3
move 1 from 7 to 1
move 8 from 8 to 4
move 8 from 4 to 2
move 3 from 7 to 3
move 6 from 5 to 6
move 15 from 3 to 1
move 21 from 6 to 1
move 4 from 2 to 6
move 5 from 6 to 5
move 1 from 2 to 6
move 1 from 4 to 5
move 1 from 4 to 3
move 1 from 8 to 6
move 4 from 5 to 7
move 18 from 1 to 4
move 2 from 5 to 7
move 6 from 7 to 6
move 1 from 3 to 2
move 6 from 1 to 2
move 3 from 3 to 9
move 3 from 9 to 4
move 1 from 8 to 3
move 1 from 6 to 5
move 6 from 2 to 5
move 1 from 5 to 9
move 1 from 3 to 5
move 2 from 6 to 8
move 2 from 1 to 4
move 5 from 4 to 6
move 15 from 4 to 9
move 5 from 9 to 1
move 2 from 6 to 2
move 6 from 6 to 3
move 1 from 8 to 6
move 6 from 5 to 9
move 3 from 6 to 5
move 2 from 4 to 7";
