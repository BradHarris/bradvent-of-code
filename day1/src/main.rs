use utils::read_lines;

#[derive(Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }
}

fn main() {
    let mut elves: Vec<Elf> = read_lines("./day1/input.txt")
        .unwrap()
        .map(|l| l.unwrap().parse::<u32>().ok())
        .fold(vec![Elf::new()], |mut acc, n| {
            if let Some(n) = n {
                let elf = acc.last_mut().unwrap();
                elf.calories.push(n);
            } else {
                acc.push(Elf::new());
            }
            return acc;
        });

    elves.sort_by_cached_key(|e| e.calories.iter().sum::<u32>());
    elves.reverse();

    let most_calories = elves.first().unwrap().calories.iter().sum::<u32>();
    println!("1 - most calories: {most_calories}");

    let top3_calories_summed = elves[0..3].iter().map(|e| e.calories.iter().sum::<u32>()).sum::<u32>();
    println!("2 - top 3 calories: {top3_calories_summed}");
}
