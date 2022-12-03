use std::str::FromStr;

use utils::read_lines;

#[derive(Debug, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(format!("{s} is not a valid RPS")),
        }
    }
}

impl RPS {
    fn score(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn fight(&self, enemy: RPS) -> usize {
        if enemy == self.win_to() {
            6
        } else if enemy == self.lose_to() {
            0
        } else {
            3
        }
    }

    fn lose_to(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn win_to(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn draw(&self) -> RPS {
        self.clone()
    }
}

enum Round {
    Lose,
    Draw,
    Win,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Round::Lose),
            "Y" => Ok(Round::Draw),
            "Z" => Ok(Round::Win),
            _ => Err(format!("{s} is not a valid round")),
        }
    }
}

fn main() {
    let score = read_lines("./day2/input.txt")
        .unwrap()
        .map(|l| {
            let l = l.unwrap();
            let (enemy, you) = l.split_once(' ').unwrap();
            (enemy.parse::<RPS>().unwrap(), you.parse::<RPS>().unwrap())
        })
        .fold(0, |acc, (enemy, you)| acc + you.score() + you.fight(enemy));

    println!("1 - score: {score}");

    let score = read_lines("./day2/input.txt")
        .unwrap()
        .map(|l| {
            let l = l.unwrap();
            let (enemy, expected) = l.split_once(' ').unwrap();
            (
                enemy.parse::<RPS>().unwrap(),
                expected.parse::<Round>().unwrap(),
            )
        })
        .fold(0, |acc, (enemy, expected)| {
            acc + match expected {
                Round::Lose => enemy.win_to().score(),
                Round::Draw => enemy.draw().score() + 3,
                Round::Win => enemy.lose_to().score() + 6,
            }
        });

    println!("2 - score: {score}");

    alternate_solution();
}

fn alternate_solution() {
    let score = read_lines("./day2/input.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .fold(0, |acc, l| {
            acc + match l.as_str() {
                "A X" => 3 + 1, // rock - rock
                "A Y" => 6 + 2, // rock - paper
                "A Z" => 3, // rock - scissors
                "B X" => 1, // paper - rock
                "B Y" => 3 + 2, // paper - paper
                "B Z" => 6 + 3, // paper - scissors
                "C X" => 6 + 1, // scissors - rock
                "C Y" => 2, // scissors - paper
                "C Z" => 3 + 3, // scissors - scissors
                _ => panic!("{l} does not match anything!"),
            }
        });
    println!("alt 1 - score: {score}");

    let score = read_lines("./day2/input.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .fold(0, |acc, l| {
            acc + match l.as_str() {
                "A X" => 3, // rock - lose - scissors
                "A Y" => 3 + 1, // rock - draw - rock
                "A Z" => 6 + 2, // rock - win - paper
                "B X" => 1, // paper - lose - rock
                "B Y" => 3 + 2, // paper - draw - paper
                "B Z" => 6 + 3, // paper - win - scissors
                "C X" => 2, // scissors - lose - paper
                "C Y" => 3 + 3, // scissors - draw - scissors
                "C Z" => 6 + 1, // scissors - win - rock
                _ => panic!("{l} does not match anything!"),
            }
        });
    println!("alt 2 - score: {score}");
}
