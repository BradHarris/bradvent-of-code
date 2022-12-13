use crate::solver::Solver;

#[derive(Default, Debug)]
pub struct Solution {
    input: String,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input.to_owned();
    }

    fn solve_part1(&self) -> String {
        "".to_string()
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
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(INPUT);
        let solution = solver.solve_part1();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(INPUT);
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const INPUT: &'static str = "";
