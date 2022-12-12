use super::Solver;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
    h: u8,
}

#[derive(Default, Debug)]
struct Grid(Vec<Vec<Point>>);

impl Grid {
    fn astar(&self, start: &Point, end: &Point) -> Vec<Point> {
        Vec::new()
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    grid: Grid,
    start: Point,
    end: Point,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.grid = Grid(
            input
                .split('\n')
                .enumerate()
                .map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            let p = Point {
                                x: x as u8,
                                y: y as u8,
                                h: c as u8,
                            };
                            if c == 'S' {
                                self.start = p.clone();
                            } else if c == 'E' {
                                self.end = p.clone();
                            }
                            p
                        })
                        .collect()
                })
                .collect(),
        );
    }

    fn solve_part1(&self) -> String {
        self.grid.astar(&self.start, &self.end).len().to_string()
    }

    fn solve_part2(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "31");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}
