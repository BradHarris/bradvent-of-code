use std::{collections::HashMap, str::FromStr};

use utils::read_lines;

#[derive(Debug)]
struct RuckSack {
    compartment_1: String,
    compartment_2: String,
}

impl FromStr for RuckSack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let partition = s.len() / 2;
        Ok(RuckSack {
            compartment_1: s[0..partition].to_string(),
            compartment_2: s[partition..s.len()].to_string(),
        })
    }
}

impl RuckSack {
    fn find_matching_item(&self) -> char {
        self.compartment_1
            .chars()
            .find(|c| self.compartment_2.contains(*c))
            .unwrap()
    }
}

fn main() {
    let lines = read_lines("./day3/input.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let item_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..53)
        .collect::<HashMap<char, usize>>();

    let score: usize = lines
        .iter()
        .map(|l| l.parse::<RuckSack>().unwrap().find_matching_item())
        .map(|i| item_map.get(&i).unwrap())
        .sum();

    println!("part 1: {score}");

    let score: usize = lines
        .chunks(3)
        .map(|g| g[0].chars().find(|c| g[1].contains(*c) && g[2].contains(*c)).unwrap())
        .map(|i| item_map.get(&i).unwrap())
        .sum();

    println!("part 2: {score}");
}
