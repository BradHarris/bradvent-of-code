use crate::solver::Solver;

#[derive(Debug)]
enum Space {
    Open,
    Tree,
}

impl From<char> for Space {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Space::Tree,
            _ => Space::Open,
        }
    }
}

#[derive(Default, Debug)]
struct Hill {
    spaces: Vec<Vec<Space>>,
}

impl Hill {
    fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut r = 0;
        self.spaces.iter().step_by(down).fold(0, |tree_count, row| {
            let space = row.get(r % row.len()).unwrap();
            r += right;
            match space {
                Space::Tree => tree_count + 1,
                Space::Open => tree_count,
            }
        })
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    input: Hill,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.input = Hill {
            spaces: input
                .lines()
                .map(|l| l.chars().map(Space::from).collect())
                .collect(),
        }
    }

    fn solve_part1(&self) -> String {
        self.input.count_trees(3, 1).to_string()
    }

    fn solve_part2(&self) -> String {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(r, d)| self.input.count_trees(*r, *d))
            .product::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "7");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "336");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "284");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "3510149120");
    }
}

const INPUT: &str = "\
.##.#.........#.....#....#...#.
.#.#.#...#.......#.............
......#..#....#.#...###.......#
.......###......#.....#..##..#.
..#...##.......#.......###.....
....###.#....###......#....#..#
......#..#....#...##...........
..#..#....#...#.....####.......
...#........#.#.......#..#...#.
......#...#........#...#..##...
#..#........#............#...##
..#..#.#....#...........#...###
#.#..#...........#.##.#.#....#.
.#.#....#...##.....#...........
.....##....#...#..............#
...#....#...#.#.#.#...#........
#....#....#.#.#..#....#..#..#..
.................#..#.....#....
#..###...#.#..#.#......#.......
...#..........#......#....#....
.#.#.........##..#.......#...#.
.#..........#...#..#...........
....##.#.......................
.......#...........#...#.......
...#...#..##...#....###..#....#
....#.#.....##...##.#.#........
...........#.#..#.#......#..#..
.....#.....#....#...#........#.
..#......#..#.........#.....#..
.........................#...#.
#...#...#....#........##....#..
#..#.#.............#..........#
.#.........#.....#..#.#.#..#.#.
#...#..#.......####.#....##....
##...##..#.#.#...#.#.....#..#.#
.#..#....#.##........#...#....#
#...#..##.#....##..#..#.#......
.#........#.....#.#....##.##.#.
...#...#........#..#.##.##.....
....................#.#.#.#...#
..####.#..##...#....#.....##...
#......#.....#.#......#.#..#.##
..#.....#..#...........##.#....
#....#........#............#...
..##....#..............#......#
..#......#.#.......####......#.
..............##....#....##.#..
.#...............#....#....#.#.
..#.#.#..#.......##.#..........
.#...#.......#.#....#.##.......
.....#.##...#...........#.#....
..#.#..#...#..##...#.#.......##
.#.....#....#.#......#.#.......
....##.........#.#.............
.......##.......#..............
..........#......#......#....##
..##.....#..#.#..........#.....
...#....#.......#....##........
.......#...........#...........
...#.#......#.#........#....#..
.....#...........#.#.#...#.#..#
.#.#...#.#.#..........#.....###
#........#...#.................
...##.....#.....#..#..#.......#
......##...........#..#....##..
.........#............##...#...
.....#.....##...##.............
.#....#..#.#.#.#...#..#..#.....
.....#..#.#..#....#..#.........
....#.....#......#...#.........
#..#..#.................#......
.###.....#...#.#........##.#...
..#...#....#.##..#.....#.#....#
..#...##.................#.#...
....##..........#..#..#..#....#
....#..##....##.....#.#....#...
.#.#.#.....##........#.##..##.#
....#..#......#..#........#....
.......#.....###.#....#.......#
#....#.......#......##.#.......
.##.#.........#.#..##..#....##.
......#........#.#....#...#....
.####.....#.........#.#......##
##....#......#....#..#.#....##.
...........###.#.....#..#......
.......#...........#...........
........###....#..#.#..........
....#........#......#..........
.........#......#..............
...#...............#......#...#
....#..##...#.........#...#....
##........#.#....#......###....
....#.......................#..
#................#.#..#......##
...#.#.....#...#...........#.##
.#....#.##......#...##.#....#..
#...#....#..............#..#..#
.......#....#.##............#.#
.....#.#.......#.#...#.........
...#.....#..##...##...#........
..#.......#..####..#..#...#....
#.#................##...##.#..#
.....#.....##.#.....#......#..#
....#.#...#.........#.........#
..#......#............#.....#..
.....#..........#.#..#..##...##
........#................#.#...
#...#.#....##...###...#.#......
.............##.#..##..........
#..#......#...........#......#.
#.#....#..........#.##....###..
.............#.........#....#..
#........#..#.#..#...#....#....
..............#..............##
.....#...#..............#.##...
#...##..#...........#..........
..#....#...#.#........#..#.#..#
..##......#...............#....
....#...#..###..#......###.#...
.......##..#.#........#....#...
..##...#.......#...#...........
.#.......#.....#.#...##..#....#
.............#.......#.#.#....#
#.......#..#..#...#.#......##..
#.##..#..#..#....##.#...###.#.#
...##...#..#..#........#.#..#..
#....##........................
##...#...#......#.#.....#..#...
......#............#....#......
#......#.......#.......##.#....
..................#..#..#.#....
..#..................##.#......
..##........#.#.....##..#..#.#.
#....#..............#....####..
#..#..........................#
..#.#.#.#....#.......#....#.#..
.....#.#........#..........#.#.
........#.....#.......#........
#.....#....#.###.....#.......#.
.....##.#...#.#..#...#.#.#.....
......##...#.#...##..........#.
.#............#.....#..#....#..
.#................#.#..#.......
....................##...##....
#.......##...#.....#..#........
.##....#.#.#.#...........#...#.
..#.#..#.#.........#...........
...#......#.....#...##.........
..........#.#.....###.#........
.............#.....##..........
.........#...####........#.####
...................#....#......
.....#.........#.#....#..#...#.
.##...#.......##.#...#.#.#..#..
.....##........#....#...#.##.#.
#...#...#.#....#..............#
#..#.##.............#..........
..#...#..#.#.##..............##
#......#.#...##..........#.##..
.##.#...#...#.........#.#......
......#........##.#..#.........
#..#.......#......#.#..#.#.....
.#..#...........#.#.##.....#...
.....................#..#.#....
........#...##......#.....##...
#.............#...##....##....#
#.#...........#....##.#......##
.....#.....#.#..........###..#.
....#...#....##....#..##.......
.#....#....#.......#.#.....#...
.#...#.......##...##........#..
......##.......#.##.#.###......
....##.......#......#..........
...................#..##.......
......................#...##...
...##....#.#..#..#.............
.#......##..........#...#......
....##..#....#..#...#...####.#.
...#.......#.......#........#.#
#.........#..#...#...##...#.#.#
....#...#.......#...#....#.....
...#.....#.##..##.#.......##.##
.......#....#........#.........
.....#...#....#..#....#....#...
.##....#...#........#...#.#...#
.......##............#..#...#..
#.#...#....#......#.#..........
.#.##...........#........#.....
.#....#.............#.#.##.....
#.......###..#...###.........#.
#..#.#.......#.........#...#..#
..........#......#........#...#
.#.#...#.##.......##...........
.....#.........#.....#.........
.........#.........#....##.#..#
.#.......##..##..#.....#...#...
.#.....##...#..#..............#
..##...#..#..#.#...#..........#
.#.......####......#......####.
##..##........#.....#........#.
..##.#..#.#....................
...........#..#...##....##.....
..#.#........#.........#....##.
..#...#..##..###.#..###........
......#..#.............#..##...
.##.........#.#..#...#.##.###..
.#...............#...........#.
.#....#........#....#........##
..#####.#.#..#.#........##...#.
###....#....#...#..............
.....#...##............#...#...
##...........##.#.##.....#.....
..............#..#.....#...#...
...................#...........
#..........##.........#........
...#.........#..#.....#..#..#..
....###.#......#......##....#..
#......#..........#...#........
...#.#...#..#..........##......
.....##.....#.#............##..
..#..#.###....#.#.#...##....#..
...#........#....##.......#....
.#.............#..##.......#...
..#.#..###..#.....#...##.......
.........#......##...#.#..#....
.............#....##....#.#....
#..#...#....#.#...#......##....
.............#.#......#.....###
#.##....#........#.............
.....#...#.####...#.....#......
....#....###....##.......#.....
..#....##..#....#.#.......#....
...#.....#....#.........#......
.#......#.#....#.#........#....
.......#......#.....#.#..#.....
#......#.........##.##.#...#...
..#.###...................#....
....#..#....##.#........#....#.
...........#..........#......#.
.#..#.#...###..........#..#...#
...#...##..#....#...#..........
.#........#.................##.
....#.......##....#...#........
#.#...##.##...#.#.......#...#..
.....#.#.##.#......#..#..##....
.....##...#.#.....#...#........
#.#.......#..#..........##.....
................#......#..#.#.#
#......#...#...................
...#.....##.#.........#.#..#..#
...#..##..##.......#....#......
....##...#....#..#...........#.
..#..#......#...#..#...........
...#.##....#...##.......#......
.......#....#..#..##..#..#....#
.#.................#.#...#.##..
.....#..................#..#.#.
...#......##...#...........#...
..#.........#....#..#...#.....#
..#...#.....#.........##.#.....
.....#.#....##...............#.
....#...#............#.........
.....#.....###............#....
..#.#.#.......#....#...........
...........##...##...#.......#.
.........###.#......#..........
.#.......#....#.....#.##..#...#
..#..................#..###....
..#....#...#......##.........#.
........#..#........#.........#
.#..#......#.........#.........
...#..##.....#....#....#.....#.
......#.#............###.....##
.......#........#.......#.#....
..#.............#..............
.............##..#.#.#....#....
.................#....#.#......
##..#.#.......#....#.....#.....
.##............##.#.......#.#..
#..#...........##......#.......
.##......#####..##.#....#.#....
.......##.....#...#........#...
.#.#.....##....#..#....#..#...#
............##.#.....##.#......
........##...###.#......#......
......#..#.#...#..#............
.........#...........#......#..
.#.........#............##.....
.#..#..#...#.#.............#...
......#.#..##...#.#...........#
#.##.......#...#.........#.....
.....#..#............#....##...
.#......#........#.............
..#...#....#..#.......###......
....#.......###.#.#...........#
.............#...##............
.##.#.#.#...........#...#....#.
............##.........#......#
...............#......#...#....
...#.....#..###..#...........#.
.#........#.....##........#.#..
....#.#.......#..#..#...##.#.#.
.......##...........#...#......
....#.#..##......#.......#.....
..#........#.#......#.#........
........#....#..#....#..##.....
.#.........##..........#.#.....
..##...##.....##......##..#....
.###.....##...........##.#...##
...#................#.......#..
#.......#.#.#..#.#.##..#...#...
.#.#.......#..#................
..#.#.#......#............#....
#.....#.###..#.#...#...........
#...........#..........#.#.#.##
..#.#...#......##.....#........
........#.......#.#...#...#....
..#..........#......###......#.
..........##.#....#.....#.##...
..#.....#......#.........#..##.
.#...#........#..#.#..#...##..#
..###........#......#.#........
..#.##.#....#.#....#.#...#.....";
