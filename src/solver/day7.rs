use std::{collections::HashMap, str::FromStr};

use super::Solver;

#[derive(Debug, Default)]
struct Directory {
    children: HashMap<String, Directory>,
    size: usize,
}

impl Directory {
    fn iter(&self) -> DirectoryIter {
        DirectoryIter::new(self)
    }
}

impl From<&[String]> for Directory {
    fn from(input: &[String]) -> Self {
        let mut dir = Directory::default();

        fn create_dir(input: &[String], dir: &mut Directory, position: &mut usize) -> usize {
            while let Some(next) = input.get(*position) {
                *position += 1;
                let com = next.parse::<Command>().unwrap();
                match com {
                    Command::CD(name) => {
                        if name == ".." {
                            return dir.size;
                        } else if name != "/" {
                            dir.size +=
                                create_dir(input, dir.children.get_mut(&name).unwrap(), position);
                        }
                    }
                    Command::LS => {}
                    Command::Dir(name) => {
                        dir.children.insert(
                            name,
                            Directory {
                                children: HashMap::new(),
                                size: 0,
                            },
                        );
                    }
                    Command::File(size) => dir.size += size,
                };
            }

            dir.size
        }

        create_dir(input, &mut dir, &mut 0);
        dir
    }
}

struct DirectoryIter<'a> {
    stack: Vec<&'a Directory>,
}

impl<'a> DirectoryIter<'a> {
    fn new(root: &'a Directory) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(d) = self.stack.pop() {
            self.stack.extend(d.children.iter().map(|(_, dir)| dir));
            Some(d)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS,
    Dir(String),
    File(usize),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {
            Ok(Command::CD(s.trim_start_matches("$ cd ").to_string()))
        } else if s.starts_with("$ ls") {
            Ok(Command::LS)
        } else if s.starts_with("dir") {
            Ok(Command::Dir(s.trim_start_matches("dir ").to_string()))
        } else {
            let (size, _name) = s
                .split_once(' ')
                .ok_or_else(|| "bad file descriptor!".to_string())?;
            let size = size.parse::<usize>().map_err(|e| e.to_string())?;
            Ok(Command::File(size))
        }
    }
}

pub struct Day7 {
    directory: Directory,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            directory: Directory::default(),
        }
    }
}

impl Solver for Day7 {
    fn parse(&mut self, input: &[String]) {
        self.directory = Directory::from(input);
    }

    fn solve_part1(&self) -> String {
        let target_size = 100000;
        self.directory
            .iter()
            .filter(|d| d.size <= target_size)
            .map(|d| d.size)
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        let total_space = 70000000;
        let target_free_space = 30000000;
        let current_used_space = self.directory.size;
        let amount_to_clean_up = target_free_space - (total_space - current_used_space);

        let mut big_dirs = self
            .directory
            .iter()
            .filter(|d| d.size >= amount_to_clean_up)
            .collect::<Vec<&Directory>>();

        big_dirs.sort_by_key(|d| d.size);

        big_dirs[0].size.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_directory_from_string_array() {
        let dir = Directory::from(&get_input()[..]);
        assert_eq!(dir.size, 48381165);
    }

    #[test]
    fn test_solution_part1() {
        let mut day7 = Day7::new();
        day7.parse(&get_input()[..]);
        let solution = day7.solve_part1();
        assert_eq!(solution, "95437");
    }

    #[test]
    fn test_solution_part2() {
        let mut day7 = Day7::new();
        day7.parse(&get_input()[..]);
        let solution = day7.solve_part2();
        assert_eq!(solution, "24933642");
    }
}
