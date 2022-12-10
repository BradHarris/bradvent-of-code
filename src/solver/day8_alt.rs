/**
 * This was my first attempt at part 1 of day 8. I started
 * by trying to over optimize which ended up making my abstraction
 * useless for part 2. So I abandoned this approach which I'm not sure
 * would have even been that much faster than the more naive solution.
 */
use std::cmp::max;

use super::Solver;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// represents the max height of neighbours given a direction
#[derive(Debug)]
struct Neighbours {
    up: i8,
    down: i8,
    left: i8,
    right: i8,
}

impl Default for Neighbours {
    fn default() -> Self {
        Self {
            up: -1,
            down: -1,
            left: -1,
            right: -1,
        }
    }
}

impl Neighbours {
    fn set_max(&mut self, height: i8, dir: Direction) {
        match dir {
            Direction::Up => self.up = max(height, self.up),
            Direction::Down => self.down = max(height, self.down),
            Direction::Left => self.left = max(height, self.left),
            Direction::Right => self.right = max(height, self.right),
        };
    }

    fn get(&self, dir: Direction) -> i8 {
        match dir {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Default, Debug)]
struct Tree {
    height: i8,
    max_neighbor_heights: Neighbours,
}

impl Tree {
    fn new(height: i8) -> Self {
        Self {
            height,
            max_neighbor_heights: Neighbours::default(),
        }
    }

    fn set_neighbor_max(&mut self, height: i8, dir: Direction) {
        self.max_neighbor_heights.set_max(height, dir);
    }

    fn get_neighbor_max(&self, dir: Direction) -> i8 {
        max(self.height, self.max_neighbor_heights.get(dir))
    }

    fn is_visible(&self) -> bool {
        self.height > self.max_neighbor_heights.up
            || self.height > self.max_neighbor_heights.right
            || self.height > self.max_neighbor_heights.down
            || self.height > self.max_neighbor_heights.left
    }
}

#[derive(Default, Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    rows: usize,
    cols: usize,
}

impl Forest {
    fn calc_max_heights(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let h_up = self
                    .get(row as isize - 1, col as isize)
                    .map(|t| t.get_neighbor_max(Direction::Up))
                    .unwrap_or(-1);

                self.get_mut(row as isize, col as isize)
                    .unwrap()
                    .set_neighbor_max(h_up, Direction::Up);

                let h_down = self
                    .get((self.rows - row) as isize, col as isize)
                    .map(|t| t.get_neighbor_max(Direction::Down))
                    .unwrap_or(-1);

                self.get_mut((self.rows - (row + 1)) as isize, col as isize)
                    .unwrap()
                    .set_neighbor_max(h_down, Direction::Down);
            }
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let h_right = self
                    .get(row as isize, col as isize - 1)
                    .map(|t| t.get_neighbor_max(Direction::Right))
                    .unwrap_or(-1);

                self.get_mut(row as isize, col as isize)
                    .unwrap()
                    .set_neighbor_max(h_right, Direction::Right);

                let h_left = self
                    .get(row as isize, (self.cols - col) as isize)
                    .map(|t| t.get_neighbor_max(Direction::Left))
                    .unwrap_or(-1);

                self.get_mut(row as isize, (self.cols - (col + 1)) as isize)
                    .unwrap()
                    .set_neighbor_max(h_left, Direction::Left);
            }
        }
    }

    fn get(&self, row: isize, col: isize) -> Option<&Tree> {
        if row < 0 || col < 0 {
            return None;
        }
        self.trees.get(row as usize)?.get(col as usize)
    }

    fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut Tree> {
        if row < 0 || col < 0 {
            return None;
        }
        self.trees.get_mut(row as usize)?.get_mut(col as usize)
    }
}

pub struct Day8 {
    forest: Forest,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            forest: Forest::default(),
        }
    }
}

impl Solver for Day8 {
    fn parse(&mut self, input: &[String]) {
        self.forest.trees = input
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| Tree::new(c.to_digit(10).unwrap() as i8))
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Vec<Tree>>>();
        self.forest.rows = self.forest.trees.len();
        self.forest.cols = self.forest.trees.first().unwrap().len();

        self.forest.calc_max_heights();
    }

    fn solve_part1(&self) -> String {
        let mut is_visible_count = 0;
        for row in 0..self.forest.rows {
            for col in 0..self.forest.cols {
                let tree = self.forest.get(row as isize, col as isize).unwrap();
                if tree.is_visible() {
                    is_visible_count += 1;
                }
            }
        }

        is_visible_count.to_string()
    }

    fn solve_part2(&self) -> String {
        "not implemented!".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        "\
30373
25512
65332
33549
35390"
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_parse() {
        let mut day8 = Day8::new();
        day8.parse(&get_input()[..]);
        println!("{:#?}", day8.forest);
    }

    #[test]
    fn test_parse2() {
        let mut day8 = Day8::new();
        day8.parse(&get_input()[..]);
        println!(
            "{:#?}",
            day8.forest
                .trees
                .iter()
                .map(|r| format!(
                    "{:?}",
                    r.iter()
                        .map(|t| format!("{}{}", t.height, t.is_visible()))
                        .collect::<Vec<String>>()
                ))
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_solution_part1() {
        let mut day8 = Day8::new();
        day8.parse(&get_input()[..]);
        let solution = day8.solve_part1();
        println!("{:#?}", solution);
    }

    #[test]
    fn test_solution_part2() {
        let mut day8 = Day8::new();
        day8.parse(&get_input()[..]);
        let solution = day8.solve_part2();
        println!("{:#?}", solution);
    }
}
