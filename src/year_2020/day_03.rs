use utils::read_lines;

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

struct Hill {
    spaces: Vec<Vec<Space>>,
}

impl Hill {
    fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut r = 0;
        self.spaces.iter().step_by(down).fold(0, |tree_count, row| {
            let space = row.get(r % row.len()).unwrap();
            r = r + right;
            match space {
                Space::Tree => tree_count + 1,
                Space::Open => tree_count,
            }
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();
    let hill = Hill {
        spaces: lines
            .map(|l| l.unwrap().chars().map(|c| Space::from(c)).collect())
            .collect(),
    };

    let part1 = hill.count_trees(3, 1);
    let part2 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(r, d)| hill.count_trees(*r, *d))
        .fold(1, |acc, trees| acc * trees);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

// use std::fmt::Display;

// impl Display for Space {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Open => write!(f, " "),
//             Self::Tree => write!(f, "Δ"),
//         }
//     }
// }

// impl Display for Hill {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.spaces.iter().fold(Ok(()), |result, row| {
//             result.and_then(|_| {
//                 write!(
//                     f,
//                     "{}\n",
//                     row.iter().map(|s| s.to_string()).collect::<String>()
//                 )
//             })
//         })
//     }
// }
