use std::{collections::HashSet, hash::Hash, thread::sleep, time::Duration};

use crate::solver::Solver;

const MIN_X: u8 = 0;
const MAX_X: u8 = 7;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        if value == '<' {
            Self::Left
        } else {
            Self::Right
        }
    }
}

#[derive(Debug)]
enum Shape {
    Flat,
    Cross,
    Angle,
    Tall,
    Block,
}

impl Shape {
    fn get_pieces(&self, x: u8) -> Vec<u8> {
        match self {
            Shape::Flat => vec![0b1111000 >> x],
            Shape::Cross => vec![0b0100000 >> x, 0b1110000 >> x, 0b0100000 >> x],
            Shape::Angle => vec![0b1110000 >> x, 0b0010000 >> x, 0b0010000 >> x],
            Shape::Tall => vec![
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
            ],
            Shape::Block => vec![0b1100000 >> x, 0b1100000 >> x],
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct P(u8, u64);

struct Rock<'a> {
    shape: &'a Shape,
    pos: P,
}

impl Rock<'_> {
    fn can_move_down(&self, rocks: &Vec<u8>) -> bool {
        // base case is rock is at y == 0 on the floor
        if self.pos.1 == 0 {
            return false;
        }

        !self
            .shape
            .get_pieces(self.pos.0)
            .iter()
            .enumerate()
            .any(|(y, p)| {
                if let Some(r) = rocks.get(y + self.pos.1 as usize - 1) {
                    r & p != 0
                } else {
                    false
                }
            })
    }

    fn can_move_left(&self, rocks: &Vec<u8>) -> bool {
        if self.pos.0 == MIN_X {
            return false;
        }

        !self
            .shape
            .get_pieces(self.pos.0 - 1)
            .iter()
            .enumerate()
            .any(|(y, p)| {
                if let Some(r) = rocks.get(y + self.pos.1 as usize) {
                    r & p != 0
                } else {
                    false
                }
            })
    }

    fn can_move_right(&self, rocks: &Vec<u8>) -> bool {
        let x = self.pos.0 + 1;

        if match self.shape {
            Shape::Flat => x + 3 == MAX_X,
            Shape::Cross => x + 2 == MAX_X,
            Shape::Angle => x + 2 == MAX_X,
            Shape::Tall => x == MAX_X,
            Shape::Block => x + 1 == MAX_X,
        } {
            return false;
        }

        !self.shape.get_pieces(x).iter().enumerate().any(|(y, p)| {
            if let Some(r) = rocks.get(y + self.pos.1 as usize) {
                r & p != 0
            } else {
                false
            }
        })
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    jets: Vec<Dir>,
    shapes: Vec<Shape>,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.jets = input.chars().map(Dir::from).collect();
        self.shapes = vec![
            Shape::Flat,
            Shape::Cross,
            Shape::Angle,
            Shape::Tall,
            Shape::Block,
        ];
    }

    fn solve_part1(&self) -> String {
        let mut jets = self.jets.iter().cycle();
        let mut shapes = self.shapes.iter().cycle();
        let mut rocks: Vec<u8> = Vec::new();

        let mut max_y = 0;
        for _ in 0..2022 {
            let mut rock = Rock {
                pos: P(MIN_X + 2, max_y + 3),
                shape: shapes.next().unwrap(),
            };

            loop {
                match jets.next().unwrap() {
                    Dir::Left => {
                        if rock.can_move_left(&rocks) {
                            rock.pos.0 -= 1;
                        }
                    }
                    Dir::Right => {
                        if rock.can_move_right(&rocks) {
                            rock.pos.0 += 1;
                        }
                    }
                }

                if rock.can_move_down(&rocks) {
                    rock.pos.1 -= 1;
                } else {
                    break;
                }
            }

            let pieces = rock.shape.get_pieces(rock.pos.0);

            for (y, section) in pieces.iter().enumerate() {
                let y = y + rock.pos.1 as usize;

                if y >= rocks.len() {
                    rocks.push(*section);
                    max_y += 1;
                } else {
                    rocks[y] = rocks[y] | section;
                }
            }
            // print_view(&rock, &rocks, max_y);
        }

        max_y.to_string()
    }

    fn solve_part2(&self) -> String {
        "".to_string()
    }
}

fn print_view(rock: &Rock, rocks: &Vec<u8>, max_y: u64) {
    // return;
    clear_terminal();
    println!("--{max_y:0>4}--");
    let max_y = max_y.max(30);
    let min_y = if max_y > 30 { max_y - 30 } else { 0 };
    let shape = rock.shape.get_pieces(rock.pos.0);
    for y in (min_y..max_y + 10).rev() {
        print!("|");
        let rocks = rocks.get(y as usize).unwrap_or(&0u8);
        let rock = if y >= rock.pos.1 && y < rock.pos.1 + shape.len() as u64 {
            shape.get(y as usize - rock.pos.1 as usize).unwrap_or(&0u8)
        } else {
            &0u8
        };

        let line = format!("{:#09b}", rock | rocks)
            .trim_start_matches("0b")
            .replace("0", ".")
            .replace("1", "X");

        print!("{line}");
        print!("|{:0>4}\n", y + 1);
        if y == 0 {
            println!("+-------+");
        }
    }
    sleep(Duration::from_millis(140));
}

/*
6
5 2
4 1
3 0
2
1

y 6 -> 6 - 3 + 2
*/

fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
    print!("\x1b[{};{}H", 0, 0);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    }

    #[test]
    fn test_parse_example() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        println!("{:#?}", solver);

        // print_view(80);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "3068");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "3147");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const INPUT: &str = "><<<><<>>>><<><<<<>>>><<<>>><<<>>>><<<><>><<<>>>><<<><>>><<>>>><>>>><>><<<>>><<<<>><>><<>>><<<<>>>><>>><<<>><<<<><<<<>>><<>>>><><>>><<<<>><<>>>><<><<<<><<>>>><<<><<><>>>><<<><<<>>><<<>>>><<<>>><<>>>><<<><<<<>>>><<>><<<<>>>><<>>>><<<>><<>>>><>>>><<<<>>><>>>><>>>><<<<>><<<>><<<>>>><<<>>>><<<><<<>>>><>><<>><>>><<<>>><>><<<<><<<<><>>><<>>>><<<<>>>><<<<><<<>>>><><<>>><<<>>>><<<>>>><><<>>><<<>>><<<<>><<<<>>>><>><<<<><<<>>><<>>>><>><<<>>>><<<>>>><<>>>><<<><>><>>><<<>><>>>><<<<>><<<>><<><<<<>>><<<>>>><>>><<<>>><<<<>>><<>><<><<<<>>>><>><>>>><>>>><<<>>>><>>><<<>>><<<<>>><<>>>><<>>><<<>>><<>>><<<>><<<>>><<<>><>>><<>><<<><<><<<<><<<<>><>><><>>><<<><>><<<>>>><<<>>>><<<>><<<>><<<>>>><>><<<><<<><>>><<<>><<<><<<<><>>><<<<>>>><>>><<<>><><<>>>><<<>><<<>>>><<<>>><<><<>><<<<><<>>>><<<>><<<<>>>><<>>><<<<>>><<>>><<<<><><<<<>>>><<>>>><>>>><<<>>><<<<>><<>>><<<>><<>>>><>>>><<<>><<<>>><<<>><<>><<>>>><<<<>><<>><<>>>><<>>><<>>><<<>>><<<<>>><<>><<<>><>>>><<<<><<>>>><<<<>>><>>><>><>><<<<><>>>><<<<><<<<>>><<<<>>>><>>><<<<><<<>>>><<<>>><><>>>><>><>><<<<>>><<<<>>>><<<<>>><<<<>><<<>>>><<<><<>>><>><>><<>>>><>><<<<>><<><>><<<>>><><<>>>><>>>><>>>><<<>>>><<<>>>><<<>>><<>>>><<<<>><<<><<<>>>><<<<>>>><<<<>><<>><<<<>><<><>>><><<<<><><>><<>>>><<<<>>>><>>>><<<>>><<>>>><>>><<>>>><<<<><<>>>><<<>>>><>>><<>>>><>>><>><<<<><><>><<<<>>>><>>>><<>>>><<<<>>>><>>>><<><<<>><<<>><<<><<<<>>>><<>>><<>>><<><<<>>><<<>>>><<<>>>><<<<><<<<><<<>>><>>>><<<>><<<>>>><<<>>>><<<<>>><<>>>><>>>><>><<<<>><<>><>><<<<>>>><<><<<><<<<>>><<<>><<<>>><>>><<<<><<>>><<>><<>>>><<>><<><<<>><<<>>>><<><<<<>>>><<<<>>>><<<>><<<>><<<<>>>><<<>><<<<>>><>>>><<<<>><><><<>>><<<><<<>><><<<>>>><<>>>><<>>>><<<>>>><<<<>><<<<>>><<>><<<<>>><<<>><<<><<<<>><>>><<<><<<<>>>><<<<>>><<><<>><><<<<><<>><>>>><<<><<<<>>><<>><<<<>><<<>>>><<>>>><<><>>>><><<<>>>><<<<><><<<<>>><<<>>><<<><<<<>>>><<<<>>>><<<<>>>><<<<><><<<<>>><<<<>><>>><>>><<<>><>>>><<<<>><>><<<<><<<><>><<>>>><<<<><<<><<<>>><<<>>>><>>><<><<>><<<<><<<>>>><<<<>><<>>>><><<<<><><<>><<<><<<<><<>>><<>><<<>>>><<<<>>><<<>>>><<<<>>>><<<<>>><<<<>><<<<>><<<>>>><><<>>>><>>>><<<<>><><<<><><<<>>>><>>>><<>><>>>><>><<<<>>><<>>>><<>><<<<><<<<>>>><>><>>><<<<>><><<<>><<><<<<>><<>><<<<><<>><>>>><<<>><<<>>>><<<>>>><<<>>>><<<>>><<><<<><<>>><<>><<>>>><<<<>><<<<><<<<>>><>><>>>><<<<>>>><<<>>><<>>><<>>><<>><<<>><>>>><>>><>><>><<>>>><><<<<>><<<<><>>><<<>>><<<<>>><<><<>>><>><<><<<<>>>><<>>><<<<>>>><<<<><<<<>><><<<<>>><<<>><<<<>>>><<>>><<>><<<<>>><<>>><<<>><>>>><>>>><><<>><<<>>>><<<<>>>><<><<<<>>>><<<><<><<<<><<<<>>><<<<><<<<><<>><<<<>>>><<>><<<<>><<>>><<<>>>><<<><<<>><<<>>><>>><>><<<>><<>><<<>><<<>><<<<>>>><<<<>><>><<>>><<<><>><<<<>>><<<<>>><><<><<<<>>><<>>>><<<<>>>><<<<><>>>><<<<>>>><<<>>>><<<<>><<>><<>>><><>><>>><>>>><>>><>>>><<<>>><<><>>><>>><><>>>><<<><<>>>><<><<<><<>>><><<<>>><<>><<<<>><<<><>><<>><<<>><<<<>><<>><<>><>>>><>>><<<<>>>><<>>><<<>>>><<<<>>>><<><>><<>><<<<>>><>>>><>>><<>><<>>>><>>>><<>>>><<<>>><<>><<<><<<<>>><<><<>>><<<>>><><>><<<<>><<><<<>><><<<<><><<<><<<><<<<>><<>>>><<>><<<>>><<<><<<<>><<>><<<>>><<<<><>>><><<<<>>><<><<><>><>>>><>>><<<<><<>>><<<>><<<>><<<>>><>><>>>><<<>>><<<>>><><>>><<><<>><<<><<<>><<>>><<>>>><<<>>><<><>><>><<<>>><<<<>><<<><>>>><<<>>>><<>>>><<<<>>>><<<>>>><<>>>><<<><<<><<<>><>>>><<>>><>>>><<>>><<<>>>><>>>><<<<>>>><>>>><<<><<<<>><<<>>>><<<>>><<<>>>><<<<>><<<>><<<>>>><<<<>>>><<<<>>>><<<>>><>>><>>>><<<<>><<>>><>>>><>>><>>><<<>>><<>>>><<<<>>>><>>><<<>><<>><<><<<>>>><>>><<><>>><<>>><<>>><<<<>><<<>>>><<<><<<<><<><<<<>>><<>>>><<<>><>><<>>><<<<>>>><<>>>><<<<>>>><<<>>><<<>>>><>>><<><<<<>><<<><<<<>>>><>>><<<<>>><<<<>><<<>>>><>><<<<><<<><<<>>>><<<>>><>>>><<<<>><<<<>>><<>>>><>>>><<<>>>><<><>>><<<><<<<><<<<>><><<<<>><>>><<>>><<<>><<<<>>>><<<<>><<<<><<<<>><<<>>>><<<<>>><<<>><<<>><<<<>><>>>><>><<<>>>><<<<>>>><<><<<>><><<<><<>>><>>>><><<<<>>>><<>><<<>>>><<<<>><<<<><>>>><<<>>><<<<><<<><>><<<>>><>><<<><<<<><<<>>>><<<<>>>><<<>>>><<<<><>><<<<>>><<<><<>>><>><<>><<<<>><<>>>><>><<>><<<><<>>><<>><<<<>><<<<>>>><<>>><<<>>><<><>>>><<>>>><<><<<>><<<>>><>><><<<<>>>><>>><<>><<><<<<>>>><<>>>><<<>><<<>>><<<<>>>><>>>><<>><>>><<<>><>><>>>><<<<><<>>>><>>>><><>>><<<>><<<>><<<><>>>><<>>>><<<<>>><<><<>>>><<<<>>>><><<<>><<<>>><<<>>>><<>>>><<<<><<<<>>><<<<>><>>><<<>>>><>>><<<<>>><<>>><<<>>>><<><<>>><<<><<<>>>><<<>>><<<><><<<<>><<<>>>><<><<<<><>>>><><<<>>><<<>><<<>>><>>><<<>>>><<><<<><<>>><<>><<>>>><<<>><<>>>><<>><<<>>>><<<>><<<>><><<><<<>><<<>>><>><>>>><<>><<>><<>><<<>>><<<<>><<>>>><<<>><<<><<><<<>>><>><<<>><<>>>><<<<>><<<>><<<>>><<>>><<>>><<<><>><<>>><<>><<<<>>><<><>>><<<>>><<>>>><<<<>><<<<>><>>><><<>>><>>>><<><<<<><<<<><>>>><<>>>><<<<>>><>>><<><<<><>>><<<>>><<<<>>>><<<><>><<<<>><<>>><<<<><>>><>>><<<<>>>><>>><<>><<<>>>><>>>><<>>>><<<<>>><<<>><>>>><<<><><<<<><<<<>>>><<<>>><><<><<<>><>><>><<<<>>><<><<<<>>><<>>>><>>><<><<<<>><<>>>><<><<<<>><<>><>><<<><<><<<>>>><<<<><<<<>>><<>>>><<<<>>><<<<>>>><<<>>>><><<<><>>>><>>><<>>>><<<>>>><>>>><>>><>><>>><>>><<<>>><>>>><>><<<<><<<><<<<>>><<<<>>><<<<>><<<><<<>><<<>>><<<>>>><<>><<>>>><<>>>><><<><><<<<><>><<<<><<<<>>>><<>>>><<<<>>><<><<<>>>><>>><<><<>>><<<<>><>>><<<>>><<>><<<<>>><<<<>>><><<<><><<<>><>>><<>>>><<>><<<<>>><<<<><<<><>><<<<>><><><<>><>>>><<<<><<<>>>><<<<>>>><<>><<<<><><<><>>>><<<>>><<>>>><<<><>>><><<<>>>><<<<><<<<>><<<>><>><<<<>>>><<<<>>><<<>><>><<>>><><<<>>>><<<<>>><<><>>><<>><<<><><<<<>>>><<<>>><<>>>><><>><><<<<>><<<<>>>><>>><><<<>>>><>><><<<<>>>><>>><<<<>><<<<>>>><<><>>>><<<<>><<<<>>>><<<>>><<<<><<<<>><>>><<>>>><<>>>><<<><<>>>><<<>>>><><>>>><<<<>><<<>><<>>><<<<><<<>>>><<>><<<><<><<<<>><<<>><><>>>><<>><<<>>>><<<<>>><<>><<<<><<>><>>><><<<<><>>>><<<>>>><<<<>><>>>><><>><<>>><<<>>>><>><<><<<<>>>><<<>><<<>><<>>><<><<<<>>><<<>><<<>>><<>>>><<>>>><<<<>>>><>><<<><<>><<>>><<<<>><<<>><<<<>>><><>>>><<>>>><>>><>><<<<>>><>>><>>><<<><<>>>><<<>>>><<>>><<<>>><<<>><<<<><>>>><<<>><<<>><<<<>><>>>><<><<<<><>>><<<<>>><<>>><<<<>>><<<><>><<<>>><<>><<<<>>>><<>>><<<<>>>><<<<><<<>><><<<>>>><<<<>>>><<<<>>><<>>>><<>><<>><<<>><<<>><<<<>><><>><>><<<><>>>><<<><<<>><<<><<<>>><<<><<<<>>>><><<>>><><<<<><<<<>>>><<<><<<>><<><<>><<<>>>><><<>>>><<<<>><>><><>>>><<<>>>><<>>>><<>>>><>>><>>><<<<><<<<><<<<><<<<>>><><<<<>>>><><>>>><<<><><<>>><<<<><<<>><<<<>><<>>><><<<<>><><<<>>>><>>>><>>><<<>><>><<<<>><>><<<>>>><<><<>><>><<<<>>><<>><<>>>><<<<>><<<<>><>><<<>>><>><><<<<>><<<<><>><<<>>>><<<<><>>>><<<<>><>>>><>>><<<>>><<>>><>>><<<<>><<><<<>>><>><>>><>>><<<<>>>><<><>>><<>>>><<<<>>>><<>><<<>><>><<<<>>>><<<>><<>><>><>><><<><>><><<>>><<<>><>>>><<<<><<<<><<<<>>><<<<>>>><<<<>>>><<<<>>>><<>><>><<<><>>><<<>>>><<<>>><<<><<>>>><>><<>>>><<><>>><<>><<<<>><>><<<<>>>><<<>>>><<<>>><<<<>>>><<><<>>>><><<>>><>><<>><<<<>>>><<><>><>><<<<>><>><<<<>>>><><<<<><>><<<>>>><<<<>>>><<>>>><<>>><<<<>>><>>>><<><<<<>><>><>><<<<>>>><<>>>><<<<>>><<><<<>><<<>>>><<>>>><>>><>><<<>>><<>>>><><<<<>>>><<<>>>><<><<<>>><><><<<>><<>>>><<<>><<><<<<>><>>><>>><><<>>><<>><<>>>><<>><><<>>><>><>>><>>>><<<<>>>><<<<>><>><<<<>><<<<><<>>>><>>>><<>><<<>>><><<<<>>>><<<<><<><<<<>>><<><>><<<<>><<>><>>>><><<<>>>><<<<>>><<<>>>><<>><<<>>><<<<><>>>><<<<>>>><<<<><<<<>>><<<>><><<<<>><<>>><<<>>>><<>>>><<<><<>><<<<><<><>>>><<<<><>><<<>><<<<>><>><<<>>>><<>><><<<>>><<<<>>>><>>>><<>><<<<><<>>>><<>>><>>><<>>>><>>><<<<>>><<<>>><<<>><<<>>><<>>><<<<>>>><<<>><<<<>>>><<<><<<<>>><<<<>><<<<>>><<<<>><>><<<>>>><>>><<>>>><<>>><<>><>>><<<<>><>>>><<<>>>><<<>>><<<<><>>><>><<>>><>>>><<<>><<>><<>>><<<><>>><<>>><>>><<>>>><>>><<<<><>>>><>><<<>><<<<>><<>>><<>>>><<<>><<<>>><<>>>><<<><<<<>>>><<<>><>>><<<>><<>>><>>><<<>>>><<><<>>>><>><<<<>>>><<><<<>>>><<<<>>><<><<<>>>><<<>><<>><<<<>>><>>>><>>>><<>>><><<<<>>>><<<<>>><<<><<<<>><<<<>><<<<>><<<>><<<<>><>>>><<<><<<<><><<>>>><<>><>><<<<>>><<<>>><<<<>>>><<<>>>><<<>><<<<><<<>>>><<>>><<>><<<><<>>>><<<<>>><>>><<<>>>><>><<>>><<<<>>><<<>>>><><<>><<>><<<<><<>><>>><<<>>><<>>>><<<<>>><<<>>><<<>>><<><<<<>>>><<<>>><><<<><>><<>>>><><>><><<>>><>>><<<<>><<<<>><>>>><><<<><><><<>><>>><<>>><<<<>>>><<><<<<>><<<><>>>><>>><<>>>><>>><><><<<>>>><<<<>>>><<>><<<<>><<><>><>>>><>><<<>>>><><<<>><<<>><<>><<<>>><><>><><<>>><<<>><<<><<<<>>>><>>>><<<<><>>><<<<>>>><>>>><<<>>>><<<<>><<<>>><<><>>><<<<><<>>><<<<><<>>>><<<<>>><>>><<<><<<>><>>><<<<>>>><<<<>>><<>>>><<<<>>>><<<>>><<<<><<<>>><>>>><<><>>><<<>>>><>><<<<><>><>>>><>><<>><>><<>>><<<<>>><>>>><<<<>><<><>>><<<<>>><>>>><>>><<>><<<<>><<><<<>>>><>>><<><<>>>><<>>><<>>>><>>><>>>><<<<><<<<>>><<<><<>>>><<<<>><<<>><<><<><<><<<><<<<><>><<>>>><<<<>>><<<>>><<<<>>><<<><<<>>><<<>>>><<<>>><<<<>><<<<>><<><<<<>>>><<<>>><<>>><>><>><<<><<<<>>><<><<<>>><<<<>>>><>>><<<>>>><<<>><<>>><<<>>><<<>><>><>>>><<<<>>>><<<>><>><<<>><<<<>>>><<>>>><<<<><<><>>><<>>><<<<>>>><<<<><<>>><<<>><>><<<<>>>><<<<>><<<><<><<><><<<<><<>>><<<><<>><<<<><<<<><<<>><>><<<<>>>><<<>><<<<><<>><<<>>>><<>><<>><>><<<<>><>>>><<<<>><<<>><<>>>><<><<<>><<<>>>><<>>>><<<<>><>>><<<<>><>>><<<>>><<>>>><<<<>><<<><>><<><<<>><<>>>><<<>>><>><>>>><<>><>><<>><<<><<<<>>>><<<<>>>><<>>>><<<<>>>><>>>><<>>><>>>><<>>><<<<>><<<<>><<<>><<>>><<<<>>>><<<<>>>><<<>>>><<>><<<>>><<>>>><<>><<>>>><<>>>><<><<<<>><<<>>><<<><<>>><<>><<<<><<>>><<<<>>>><<><<<<><>><<>>><<<<>>>><<<<><<<>>><><>>>><>><<<><>>>><<<>>>><>>><<><<>><><<<<>>>><<>>>><<<><<>>>><>>>><<<<><<<<>>>><<><>>>><>>><><<>>><<<>>><<<<><>><<<>><<<><<<><<<<>><<<>>><<<<>>>><<<>>>><<<>>><<<>>><>>><<>><<<>>><<><<<><>>>><>>><<<<>>><<><<<<>>>><<<><<<<>><<>>>><<>>><<<<>>>><<>>><><<<>>><<>><<>><<>>>><<<<><<<>>><<<<>>><<<<>>>><<<>>>><<<<>>><<<><>>><>>><<>>>><>>><<>>><<>><<>>>><<>>><<<<>>>><<<<>><<<>>>><<<<>>>><<<<>><<<><>><<>><>>><<<<><<<>><><<<>>>><<<>>><<>>><<>>>><<<<>>>><<<><<<>>><<<>><<<>><<<<>>><<<<>><<<>>><<<><<<><<>><><>><<<>>><<<>>><<>>><<<>>><<>>><<<<>><<<>><>><<<<>><<<<>><<>><<<>><<<<>>>><<<><<<<><<<<>><>>><<>><<><<>>><<<><<>>><<<><<<<>><>>><<>><>>><<>>>><<>>>><<<<>><<><<>>><<<>>>><<><<<<>>>><><<<<>>><<<<>><<<<>><<>>><<<<>>>><>>><<<<>><<<<>><><<<<>>>><<<>>><<>><<<<>>><<<><<<>>><>><<>>>><<>>><>><>><<<<>>>><><<<<><>>><<><<>><<<>>><>><<<>>>><<<<>>><<<>><<<<>>>><>><<<>>>><<<>>><<>>>><<>><<<<>>><<>>><<<>><<><>><>><<>><<<><><>>><<<>><<<>><<<<>>><>>><><<<>><<<>>>><<<><<>>>><>>>><>><>>><<<<><<<>><<<<>><<><<<<>>>><<>><<>>>><<<><<<>>><<<<>>><<<>>>><<>>>><<<>>><<<<>><<<>>><>>><>>>><<<>><<<<><<<<><><<<>>><><<<<>><<<>>>><<<<>>>><<><<><<<><<<><><<>>";
