use std::{collections::HashSet, fmt::Display, primitive::f32, str::FromStr};

use super::Solver;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("UNKNOWN DIRECTION: {s}")),
        }
    }
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    amt: usize,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_once(' ').unwrap();
        Ok(Self {
            dir: dir.parse()?,
            amt: amt.parse().map_err(|e| format!("{:?}", e))?,
        })
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position(isize, isize);

impl Position {
    fn move_closer_to(&mut self, other: &Position) {
        if self.distance_to(other) >= 2.0 {
            self.0 += (other.0 - self.0).signum();
            self.1 += (other.1 - self.1).signum();
        }
    }

    fn move_by_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.1 += -1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 += -1,
            Direction::Right => self.0 += 1,
        };
    }

    fn distance_to(&self, other: &Position) -> f32 {
        f32::sqrt(
            f32::powf((other.0 - self.0) as f32, 2.0) + f32::powf((other.1 - self.1) as f32, 2.0),
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Default)]
pub struct Solution {
    input: Vec<Move>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input.split('\n').map(|l| l.parse().unwrap()).collect();
    }

    fn solve_part1(&self) -> String {
        let mut head = Position(0, 0);
        let mut tail = Position(0, 0);
        let mut tail_pos = HashSet::new();
        tail_pos.insert(tail.clone());

        for m in self.input.iter() {
            for _i in 0..m.amt {
                head.move_by_dir(&m.dir);
                tail.move_closer_to(&head);
                tail_pos.insert(tail.clone());
            }
        }

        tail_pos.len().to_string()
    }

    fn solve_part2(&self) -> String {
        let rope_len = 10;
        let mut rope = vec![Position(0, 0); rope_len];
        let mut tail_pos = HashSet::new();
        tail_pos.insert(rope.last().unwrap().clone());

        for m in self.input.iter() {
            for _i in 0..m.amt {
                rope.first_mut().unwrap().move_by_dir(&m.dir);
                for j in 1..rope_len {
                    let prev = rope.get(j - 1).unwrap().clone();
                    let cur = rope.get_mut(j).unwrap();
                    cur.move_closer_to(&prev);
                }
                tail_pos.insert(rope.last().unwrap().clone());
            }
        }

        tail_pos.len().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input1<'a>() -> &'a str {
        "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input1());
        println!("{:#?}", solver.input);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(get_input1());
        let solution = solver.solve_part1();
        assert_eq!(solution, "13");
    }

    fn get_input2<'a>() -> &'a str {
        "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
    }
    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(get_input2());
        let solution = solver.solve_part2();
        assert_eq!(solution, "36");
    }
}
