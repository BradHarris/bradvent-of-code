use super::Solver;

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
        ""
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        println!("{:#?}", solver.input);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}
