use std::str::FromStr;

use super::Solver;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(isize),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else {
            let (_, amt) = s.split_once(' ').ok_or_else(|| "bad command".to_string())?;
            let amt = amt
                .parse()
                .map_err(|_| "failed to parse addx number".to_string())?;
            Ok(Self::Addx(amt))
        }
    }
}

#[derive(Debug, Clone)]
struct CPUState {
    register: isize,
    add: Option<isize>,
}

#[derive(Default)]
pub struct Day10 {
    input: Vec<CPUState>,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 { input: Vec::new() }
    }
}

impl Solver for Day10 {
    fn parse(&mut self, input: &[String]) {
        self.input = input
            .iter()
            .map(|l| l.parse().unwrap())
            .flat_map(|c| match c {
                Command::Noop => vec![None],
                Command::Addx(amt) => vec![None, Some(amt)],
            })
            .scan(
                CPUState {
                    register: 1,
                    add: None,
                },
                |cpu, add| {
                    // this add amount is from the previous cycle
                    // so we do it here which is equivelent to doing
                    // it at the end of the previous cycle
                    if let Some(amt) = cpu.add {
                        cpu.register += amt;
                    }
                    // queue up the next possible add
                    cpu.add = add;
                    Some(cpu.clone())
                },
            )
            .collect();
    }

    fn solve_part1(&self) -> String {
        let signal_strength = self
            .input
            .iter()
            .skip(19) // we skip 19 because the number of cycles starts at 1
            .step_by(40)
            .enumerate()
            .fold(0_isize, |acc, (i, cpu)| {
                acc + (((i as isize * 40) + 20) * cpu.register)
            });

        // println!("{:#?}", out);
        signal_strength.to_string()
    }

    fn solve_part2(&self) -> String {
        let signal_strength = self
            .input
            .chunks(40)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(i, cpu)| {
                        let r = cpu.register;
                        if i as isize >= r - 1 && i as isize <= r + 1 {
                            "#"
                        } else {
                            "."
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("\n\n{signal_strength}\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_parse() {
        let mut day10 = Day10::default();
        day10.parse(&get_input()[..]);
        println!("{:#?}", day10.input);
    }

    #[test]
    fn test_solution_part1() {
        let mut day10 = Day10::new();
        day10.parse(&get_input()[..]);
        let solution = day10.solve_part1();
        assert_eq!(solution, "13140");
    }

    #[test]
    fn test_solution_part2() {
        let mut day10 = Day10::new();
        day10.parse(&get_input()[..]);
        let solution = day10.solve_part2();
        assert_eq!(
            solution,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
