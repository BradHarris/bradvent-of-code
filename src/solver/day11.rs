use std::{cell::RefCell, str::FromStr};

use super::Solver;

#[derive(Debug, Clone)]
enum Op {
    Sum,
    Product,
}

#[derive(Debug, Clone)]
enum Target {
    Me,
    Amount(usize),
}

#[derive(Debug, Clone)]
struct Operation {
    op: Op,
    target: Target,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        let amount = match self.target {
            Target::Me => old,
            Target::Amount(amt) => amt,
        };

        match self.op {
            Op::Sum => old + amount,
            Op::Product => old * amount,
        }
    }
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, amt) = s.split_once(' ').unwrap();
        let op = if op == "+" { Op::Sum } else { Op::Product };
        let target = if amt == "old" {
            Target::Me
        } else {
            Target::Amount(amt.parse().unwrap())
        };
        Ok(Self { op, target })
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    items: RefCell<Vec<usize>>,
    operation: Operation,
    test_divisible: usize,
    test_true_to_monkey: usize,
    test_false_to_monkey: usize,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().split('\n').collect::<Vec<&str>>();

        let index = lines[0]
            .trim()
            .trim_start_matches("Monkey ")
            .trim_end_matches(':')
            .parse()
            .map_err(|_| "bad monkey index".to_string())?;

        let items = lines[1]
            .trim()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(|i| i.parse::<usize>().map_err(|_| "bad item".to_string()))
            .collect::<Result<Vec<usize>, String>>()?;

        let operation = lines[2]
            .trim()
            .trim_start_matches("Operation: new = old ")
            .parse()
            .unwrap();

        let test_divisible = lines[3]
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse()
            .unwrap();

        let test_true_to_monkey = lines[4]
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse()
            .unwrap();

        let test_false_to_monkey = lines[5]
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse()
            .unwrap();

        Ok(Self {
            index,
            items: RefCell::new(items),
            operation,
            test_divisible,
            test_true_to_monkey,
            test_false_to_monkey,
        })
    }
}

#[derive(Default, Clone)]
pub struct Solution {
    input: Vec<Monkey>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input
            .split("\n\n")
            .map(|c| c.parse::<Monkey>().unwrap())
            .collect();
    }

    fn solve_part1(&self) -> String {
        let monkeys = self.input.clone();
        let mut monkey_business = vec![0; monkeys.len()];
        let num_roundds = 20;
        for _ in 0..num_roundds {
            for m in monkeys.iter() {
                for i in m.items.borrow_mut().drain(0..) {
                    monkey_business[m.index] += 1;
                    let worry = m.operation.apply(i) / 3;
                    if worry % m.test_divisible == 0 {
                        monkeys[m.test_true_to_monkey]
                            .items
                            .borrow_mut()
                            .push(worry);
                    } else {
                        monkeys[m.test_false_to_monkey]
                            .items
                            .borrow_mut()
                            .push(worry);
                    }
                }
            }
        }

        monkey_business.sort();
        monkey_business.reverse();
        let solution = monkey_business[0] * monkey_business[1];
        solution.to_string()
    }

    fn solve_part2(&self) -> String {
        let monkeys = self.input.clone();

        // get common denominator for all monkey divisors so we
        // can use the modulo of this LCD to keep worry low
        let common_denom: usize = monkeys.iter().map(|m| m.test_divisible).product();

        let mut monkey_business = vec![0_u64; monkeys.len()];
        let num_roundds = 10000;
        for _ in 0..num_roundds {
            for m in monkeys.iter() {
                for i in m.items.borrow_mut().drain(0..) {
                    monkey_business[m.index] += 1;
                    let worry = m.operation.apply(i) % common_denom;
                    if worry % m.test_divisible == 0 {
                        monkeys[m.test_true_to_monkey]
                            .items
                            .borrow_mut()
                            .push(worry);
                    } else {
                        monkeys[m.test_false_to_monkey]
                            .items
                            .borrow_mut()
                            .push(worry);
                    }
                }
            }
        }

        monkey_business.sort();
        monkey_business.reverse();
        let solution = monkey_business[0] * monkey_business[1];
        solution.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }

    #[test]
    fn debug_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        println!("{:#?}", solver.input);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "10605");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "2713310158");
    }
}
