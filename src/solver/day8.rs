use super::Solver;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Tree = u8;

#[derive(Default, Debug)]
struct Forest {
    trees: Vec<Vec<u8>>,
    rows_len: usize,
    cols_len: usize,
}

impl Forest {
    fn iter_dir<'a>(&'a self, row: usize, col: usize, dir: &'a Direction) -> ForestDirectionalIter {
        ForestDirectionalIter::new(self, row, col, dir)
    }

    fn get(&self, row: usize, col: usize) -> Option<&Tree> {
        self.trees.get(row as usize)?.get(col as usize)
    }
}

struct ForestDirectionalIter<'a> {
    forest: &'a Forest,
    row: isize,
    col: isize,
    dir: &'a Direction,
}

impl<'a> ForestDirectionalIter<'a> {
    fn new(forest: &'a Forest, row: usize, col: usize, dir: &'a Direction) -> Self {
        Self {
            forest,
            row: row as isize,
            col: col as isize,
            dir,
        }
    }
}

impl<'a> Iterator for ForestDirectionalIter<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
        };

        if self.row < 0 || self.col < 0 {
            return None;
        }

        self.forest.get(self.row as usize, self.col as usize)
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
                    .map(|c| c.to_digit(10).unwrap() as Tree)
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Vec<Tree>>>();
        self.forest.rows_len = self.forest.trees.len();
        self.forest.cols_len = self.forest.trees.first().unwrap().len();
    }

    fn solve_part1(&self) -> String {
        let visible_trees = self
            .forest
            .trees
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter(move |(c, tree)| {
                    vec![
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .iter()
                    .any(|dir| {
                        let mut iter = self.forest.iter_dir(r, *c, dir).peekable();

                        if iter.peek().is_none() {
                            true
                        } else {
                            iter.all(|t| tree > &t)
                        }
                    })
                })
            })
            .count();

        visible_trees.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut scenic_scores = self
            .forest
            .trees
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, tree)| {
                        let scores = vec![
                            Direction::Up,
                            Direction::Down,
                            Direction::Left,
                            Direction::Right,
                        ]
                        .iter()
                        .map(|dir| {
                            let iter = self.forest.iter_dir(r, c, dir);

                            let mut score = 0;
                            for t in iter {
                                score += 1;
                                if t >= tree {
                                    break;
                                }
                            }
                            score
                        })
                        .collect::<Vec<usize>>();

                        scores.iter().product()
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<usize>>();

        scenic_scores.sort();

        scenic_scores.last().unwrap().to_string()
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
                .map(|r| r
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
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
