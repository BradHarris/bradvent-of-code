use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    hash::Hash,
};

use crate::solver::Solver;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i16, i16);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Point {
    position: Position,
    height: u8,
}

#[derive(Default, Debug)]
struct Grid(Vec<Vec<Point>>);

impl Grid {
    fn get(&self, pos: Position) -> Option<&Point> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        self.0.get(pos.1 as usize)?.get(pos.0 as usize)
    }

    fn neighbors(&self, cur: Position) -> Vec<Position> {
        if let Some(Point {
            position: Position(x, y),
            height,
        }) = self.get(cur)
        {
            [(*x, *y - 1), (*x, *y + 1), (*x - 1, *y), (*x + 1, *y)]
                .iter()
                .filter_map(|(x, y)| {
                    self.get(Position(*x, *y))
                        .filter(|p| p.height as i16 - *height as i16 <= 1)
                        .map(|p| p.position)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn heuristic(&self, a: Position, b: Position) -> i16 {
        let Position(x1, y1) = a;
        let Position(x2, y2) = b;
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    fn astar(&self, start: &[Position], end: Position) -> Vec<Position> {
        let mut frontier: BinaryHeap<WeightedPosition> =
            BinaryHeap::from_iter(start.iter().map(|position| WeightedPosition {
                position: *position,
                weight: 0,
            }));
        let mut came_from: HashMap<Position, Option<Position>> =
            start.iter().map(|start| (*start, None)).collect();
        let mut cost_so_far: HashMap<Position, i16> =
            start.iter().map(|start| (*start, 0i16)).collect();

        while let Some(current) = frontier.pop() {
            if current.position == end {
                break;
            }

            for next in self.neighbors(current.position) {
                let new_cost = cost_so_far.get(&current.position).unwrap().to_owned() + 1;
                if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                    cost_so_far
                        .entry(next)
                        .and_modify(|e| *e = new_cost)
                        .or_insert(new_cost);

                    let priority = new_cost + self.heuristic(next, end);
                    frontier.push(WeightedPosition {
                        position: next,
                        weight: priority,
                    });
                    came_from.insert(next, Some(current.position));
                }
            }
        }

        let mut path = Vec::new();
        if !came_from.contains_key(&end) {
            return path;
        }

        let mut current = end;
        while !start.contains(&current) {
            path.push(current);
            if let Some(Some(pos)) = came_from.get(&current) {
                current = *pos;
            }
        }

        path.reverse();
        path
    }
}

#[derive(PartialEq, Eq)]
struct WeightedPosition {
    position: Position,
    weight: i16,
}

impl Ord for WeightedPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl PartialOrd for WeightedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight).map(|c| c.reverse())
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    grid: Grid,
    start: Position,
    alt_starts: Vec<Position>,
    end: Position,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.grid = Grid(
            input
                .lines()
                .enumerate()
                .map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            let mut p = Point {
                                position: Position(x as i16, y as i16),
                                height: c as u8,
                            };
                            if c == 'a' {
                                self.alt_starts.push(p.position);
                            }
                            if c == 'S' {
                                self.alt_starts.push(p.position);
                                self.start = p.position;
                                p.height = 0;
                            } else if c == 'E' {
                                self.end = p.position;
                                p.height = 25;
                            } else {
                                p.height -= 97;
                            }
                            p
                        })
                        .collect()
                })
                .collect(),
        );
    }

    fn solve_part1(&self) -> String {
        self.grid.astar(&[self.start], self.end).len().to_string()
    }

    fn solve_part2(&self) -> String {
        self.grid
            .astar(&self.alt_starts, self.end)
            .len()
            .to_string()
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.0.iter().for_each(|r| {
            r.iter().for_each(|p| {
                write!(f, " {:0>2} ", p.height).unwrap();
            });
            writeln!(f).unwrap();
        });

        Ok(())
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
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "31");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "29");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "534");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "525");
    }
}

const INPUT: &str = "\
abccccaaaaaaaaaaaaaccaaaaaaaacccccccccaaaaaaaaccccccccaaacaaacccccccaaaaaaccccccccccccccccccccccaaaacccccccccccacccccccccccccccccccccccccccccccccccccccccccccccaaaa
abccccaaaaacaaaaaaccccaaaaaaccccccccccaaaaaaacccccccccaaaaaaacccccaaaaaaaaaacccccccccccccccccccaaaaaacccccccccaaaaaaaaccccccccccccccccccccccccccccccccccccccccaaaaa
abcccaaaaaccaaaaaaccccaaaaaaccccccaacccaaaaaacccccccccaaaaaacccaaaaaaaaaaaaaaacaaccacccccccccccaaaaaaccccccccccaaaaaacccccccccccccccccccccccccccccccccccccccccaaaaa
abccccccaaccaaaaaaccaaaaaaaaccccccaaacaaaacaaacccccccaaaaaaaaccaaaaaaaaaaaaaaacaaaaacccccccccccccaaccccccccccccaaaaaaccccccccccccccccccccccccccccacccccccccccaaaaaa
abccccccccccaaccaaccaaaaccaacccccccaaaaaaaccccccccccaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaacccccccaaaaccccccccccccccccaaaaaaaacccccccccccccccccccccccccccaaccccccccccccccaa
abcccccccaaaaacccaaaaaaaacccaaccccaaaaaaccccccccccccaaaaaaaaaaaaaaaacaaaaaaaccaaaaaaccccccaaaaaccccccccccccccaaaaaaaaaaccaccccccccccccccccccccccccaccccccccccccccca
abcccccccaaaaacccaaaaaaaaccaaaaccaaaaaaaaccccccccccccccaaacaaaaaaaaacaaaaaacccccaaaacccccaaaaaaccccccccccccccaaaaaaaaaaaaacccccccccccccccllllllccccdccccccccccccccc
abccccccaaaaaacccccaaaaccccaaaaacaaaaaaaaccccccccccccccaaacccccaaaccccaaaaaacccaaccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccccklllllllllcddddccaccaaaccccc
abccccccaaaaaacccccaaaaaaaaaaaaaaaccaaccccccaacaacccccccaaccccccccccccaaacaacccccccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccckklllppllllcddddddddaaaaccccc
abccccccaaaaaaccccaaacaaaaaaaaaaaaccaaccccccaaaaaccccccccccccccccccccccccccccccccccccccccccaaccccccaaccccccccccccaacaaaaaaaccccccccccckklpppppplllmdddddddddacccccc
abccccccccaaacccccaacccaccaaaaaaccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkkppppppplmmmmmmddddddaacccc
abccccaaacaaacccccccccccccaaaaaaccccccccccccaaaaaacccccccccccccccccaaaccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkppppuppppmmmmmmmmddeeeacccc
abccccaaaaaaacccccccccccccaaaaaacccaccccccccaaaaaacccccccccccccccccaaaacccccccccccccccccccccaaacccaaaacccccccccccaaaacaaaccccccccccckkkpppuuuuuppqqmmmmmmmmeeeacccc
abcccccaaaaaaccccccccccccaaaaaaaacaaccccccccccaaaccccccccccccccccccaaaaccccccccccccccccccccaaaaccccccccccccccccccaaaaaaaacccccccccckkkkpppuuuuupqqqqqqqmmmmeeeccccc
abcccccaaaaaaaacccccccccccaccccaaaaacccccccccccccccccccccccccccccccaaaccccccccccccccaaaccccaaaacccccccccccccaaccaaaaaaaaccccccccckkkkkrrpuuuxuuuqqqqqqqqmmmmeeccccc
abccccaaaaaaaaaccccccccccccccaaaaaacccccccacaacccccccccccccccccccccccccccccccccccccaaaaaacccaaaccccccccccaaaaccaaaaaaacccccccccckkkkrrrrruuuxxuvvvvvvqqqqnnneeccccc
abcccaaaaaaaaaaccccccccccccccaaaaaaaacccccaaaaacccccccccccccccaaaaaccccccccccccccccaaaaaaccccccccccccccccaaaaaaaaaaaaacccccccccjjjkrrrrruuuxxxxvvvvvvvqqqnnneeccccc
abcaaaaacaaacccccccccccccccccaaaaaaaacccccaaaaaccaacccccccccccaaaaaccccccccccccccccaaaaaccccccccccccccccccaaaaaccaaaaaacccccccjjjrrrrruuuuuxxxyvyyyvvvqqqnneeeccccc
abcaaaaacaaaccaaccccccccccccccccaacccccccaaaaaaaaaaaccccccccccaaaaaaccccccccccccccccaaaaaccccccccccccccccaaaaacccaaaaaaaacaaacjjjrrrtttuuxxxxxyyyyyvvvqqnnneeeccccc
abaaaaaccaacccaaaccaacccaaccccccaccccccccaaaaaaaaaacccccccccccaaaaaaccccccccccccccccaacaacccccccccccccccccccaacccaaccccaaaaaacjjjrrrtttxxxxxxxyyyyyvvvrrnnneeeccccc
SbaaaaacccccccaaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccaacccaaaaaacjjjrrrtttxxxEzzzzyyyvvvrrnnneeecccccc
abcaaaaacccccccaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccccaaccccccccccccccccccccccccccccaaccccccccccaaccccacaaaacaaaaaaajjjrrrtttxxxxxyyyyyvvvrrrnnnfffcccccc
abcaacccccccaaaaaaaacccaaaaccccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccccccccaaaaaccccccccccaaccccaaaaaaaaaaaaaajjjqqqttttxxxxyyyyyyvvrrrnnnfffcccccc
abccccccccccaaaaaaaaaccccccccccccccccccccaaaaaaaaaaaaacccccccccccccccccaacccccccccccccccccccaaaaaccccccaacaaaaaccaaaacaaaaaaaacjjjqqqqttttxxyywyyyywvrrnnnfffcccccc
abccccccccccaaaaaaaaaacccccccccccccccccccaaaaaaaaacaaacccccccccccccaaacaacccccccccccccccccccaaaaaccccccaaaaaaaaccaaaaccccaaacccjjjjqqqqtttxwywwwyywwwrrnnnfffcccccc
abcccccccccccccaaaaaaacccccccccccccccccccaaaaaaaaaaaaaaaacccccccccccaaaaaccccccccccccccccccaaaaacccaaccccaaaaccccaacaacccaaaccccjjjiqqqtttwwywwwwwwwwrrroofffcccccc
abcccccccccccccaaaccccccccccccccccccccccaaaaaaaaaaaaaaaaaccccccccccccaaaaaacccccccccccccccccccaaacaaaccccaaaaaccccccccccccccccccciiiiqqqttwwwwwswwwwrrrroofffcccccc
abcccccccccccccaaccccccccccccaaaacccccccaaaaaaaaccaaaaacccccccccccccaaaaaaacccccccccccccccccccaaaaaaacccaaacaacccccaaaaacccccccccciiiqqqttwwwwsssssrrrrroofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccacaaacccaaaaaaccccccaaccccaaaaaaccccccccaacaaccccccccaaaaaaccccaaaacacccccaaaaacccccccccciiiqqqtsswsssssssrrrrooofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccccaaaccaaaaaaaccccccaaaaccaacaaaccccccccaaaaacccccccccaaaaaaaaccaaacacccccaaaaaacccccccccciiqqqssssssspposrrroooofffaccccc
abccccaaacccccccccccccccccccccaaacccccccccccccccaaacaaaccccaaaaaacccccaaaccccccccaaaaaacccccccaaaaaaaaaaaaaaaaaccccaaaaaaccccccaccciiiqqpsssssppppooooooogffaaccccc
abccccaaaaaacccaaaccccccccccccccccccccccccccccccccccccaccccaaaaacccccccccccccccccaaaaaaccccccaaaaaaaaaaaaaaaaaaccccaaaaaacccaaaaccciiiqqppppppppppoooooogggfaaacccc
abcccaaaaaaacccaaaccccccccccccccccccccccccccccccccccccccccccaaaaaccccccccccccccccaaaaaaccccccaaacaaaccccaaaaaacccccccaacccccaaaaaacciiipppppppphgggggggggggaaaacccc
abccaaaaaaaacccaaacaaacccccccccccccccccccccaacccccccccccccccaacaacccccaacccccccccccaaacccccccccccaaacccccaaaaacccccccccccccccaaaaacciiihppppphhhhgggggggggaaccccccc
abccaaaaaaacaaaaaaaaaacccccccccccccccccccccaaaccccccacccccccccccccccccaaccccccccccccccccccccccccaaaaccccaaaaaaccccccccccccccaaaaacccciihhhhhhhhhhgggggggccaaccccccc
abccccaaaaaaaaaaaaaaacccccccccccccccccccaaaaaaaaccccaaacaaaccccccccccaaaaccaaccccccccaacaacccccaaaaaaacccaacccccccccccccccccaacaaccccchhhhhhhhhaaaacccccccccccccccc
abccccaaaaaacaaaaaaaccccccccccccccccccccaaaaaaaaccccaaaaaaaccccccccccaaaaaaaacaccccccaaaaaccccccaaaaacccccccccccccccccccccccccccccccccchhhhhhacaaaaaccccccccccccccc
abccccaaccccccaaaaaacccccccccccccaaccccccaaaaaacccccaaaaaaccccccaaaaaaaaaaaaaaaccccccaaaaaacccaaaaaaacccccccccccccccccccccccccccccccccccccaaaaccaaacccccccccccaaaca
abccccccccccccaaaaaaaccccccccccccaaccccccaaaaaacccaaaaaaaaccccccaaaaaaaaaaaaaacccccccaaaaaacccaaaaaaaaccccccaaacccccccccccccccccccccccccccaaaaccccccccccccccccaaaaa
abccaaacccccccaaacaaacccccccccaaaaaaaacccaaaaaacccaaaaaaaaacccccaaaaaaaaaaaaaacccccccaaaaaccccaaaaaaaaccccccaaaaccccccccccccccccccccccccccaaaccccccccccccccccccaaaa
abcaaaacccccccaaccccccccccccccaaaaaaaacccaaccaacccaaaaaaaaaaccccccccaaaaaaacaacccccccccaaaccccccaaacaaccccccaaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaa";
