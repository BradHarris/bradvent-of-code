use utils::read_lines;

struct Password {
    start: u16,
    end: u16,
    letter: char,
    pass: String,
}

impl Password {
    fn is_valid_part1(&self) -> bool {
        let count = self
            .pass
            .chars()
            .fold(0, |acc, c| if c == self.letter { acc + 1 } else { acc });
        count >= self.start && count <= self.end
    }

    fn is_valid_part2(&self) -> bool {
        let chars: Vec<char> = self.pass.chars().collect();
        match (
            chars.get(self.start as usize - 1),
            chars.get(self.end as usize - 1),
        ) {
            (Some(a), Some(b)) => {
                a.to_owned() == self.letter && b.to_owned() != self.letter
                    || b.to_owned() == self.letter && a.to_owned() != self.letter
            }
            (Some(a), None) => a.to_owned() == self.letter,
            (None, Some(b)) => b.to_owned() == self.letter,
            _ => false,
        }
    }
}

impl From<String> for Password {
    fn from(input: String) -> Self {
        let (min, rest) = input.split_once('-').unwrap();
        let min = min.parse::<u16>().unwrap();
        let (max, rest) = rest.split_once(' ').unwrap();
        let max = max.parse::<u16>().unwrap();
        let rest = rest.split(": ").collect::<Vec<&str>>();
        let letter = rest[0].chars().next().unwrap();
        let pass = rest[1].to_string();
        Password {
            start: min,
            end: max,
            letter,
            pass,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();
    let passwords: Vec<Password> = lines.map(|l| l.unwrap().into()).collect();
    let valid_password_count_part1 = passwords.iter().filter(|p| p.is_valid_part1()).count();
    let valid_password_count_part2 = passwords.iter().filter(|p| p.is_valid_part2()).count();

    println!(
        "part 1: number of valid passwords: {}",
        valid_password_count_part1
    );
    println!(
        "part 2: number of valid passwords: {}",
        valid_password_count_part2
    );
}
