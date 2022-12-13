use std::collections::HashMap;

use utils::read_lines;

#[derive(Debug)]
struct Passport {
    byr: usize,          // birth_year
    iyr: usize,          // issue_year
    eyr: usize,          // expiration_year
    hgt: String,         // height
    hcl: String,         // hair_color
    ecl: String,         // eye_color
    pid: String,         // passport_id
    cid: Option<String>, // country_id
}

impl TryFrom<String> for Passport {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bits: Vec<&str> = value.trim().split(" ").collect();
        let mut map = HashMap::new();
        for bit in bits {
            if let Some((key, val)) = bit.split_once(":") {
                map.insert(key, val);
            } else {
                return Err(format!("{value} had invalid entry: {bit}"));
            }
        }

        Ok(Passport {
            byr: map
                .get("byr")
                .ok_or("missing byr")?
                .to_owned()
                .parse::<usize>()
                .map_err(|x| format!("failed to parse byr: {x}"))?,
            iyr: map
                .get("iyr")
                .ok_or("missing iyr")?
                .to_owned()
                .parse::<usize>()
                .map_err(|x| format!("failed to parse iyr: {x}"))?,
            eyr: map
                .get("eyr")
                .ok_or("missing eyr")?
                .to_owned()
                .parse::<usize>()
                .map_err(|x| format!("failed to parse eyr: {x}"))?,
            hgt: map.get("hgt").ok_or("missing hgt")?.to_string(),
            hcl: map.get("hcl").ok_or("missing hcl")?.to_string(),
            ecl: map.get("ecl").ok_or("missing ecl")?.to_string(),
            pid: map.get("pid").ok_or("missing pid")?.to_string(),
            cid: map.get("cid").map(|s| s.to_string()),
        })
    }
}

static VALID_ECL: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr >= 1920
            && self.byr <= 2002
            && self.iyr >= 2010
            && self.iyr <= 2020
            && self.eyr >= 2020
            && self.eyr <= 2030
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_hgt_valid(&self) -> bool {
        if self.hgt.ends_with("in") {
            if let Ok(h) = self.hgt.trim_end_matches(&['i', 'n']).parse::<u32>() {
                return h.ge(&59) && h.le(&76);
            }
        } else if self.hgt.ends_with("cm") {
            if let Ok(h) = self.hgt.trim_end_matches(&['c', 'm']).parse::<u32>() {
                return h.ge(&150) && h.le(&193);
            }
        }
        return false;
    }

    fn is_hcl_valid(&self) -> bool {
        self.hcl.starts_with("#")
            && self.hcl.len() == 7
            && self.hcl[1..7].chars().all(|c| c.is_ascii_hexdigit())
    }

    fn is_ecl_valid(&self) -> bool {
        VALID_ECL.contains(&&self.ecl[..])
    }

    fn is_pid_valid(&self) -> bool {
        self.pid.len() == 9 && self.pid.parse::<u64>().is_ok()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[0]).unwrap();
    let passports: Vec<Result<Passport, String>> = lines
        .fold(vec!["".to_string()], |mut acc, l| {
            let line = l.unwrap();
            if line == "" {
                acc.push("".to_string())
            } else {
                acc.last_mut().unwrap().push_str(&format!(" {line}"));
            }
            acc
        })
        .iter()
        .map(|s| Passport::try_from(s.to_owned()))
        .collect();

    let valid_passport_count1 = passports.iter().filter(|p| p.is_ok()).count();
    let valid_passport_count2 = passports
        .iter()
        .filter(|p| if let Ok(p) = p { p.is_valid() } else { false })
        .count();

    println!("number of valid passports part1: {valid_passport_count1}");
    println!("number of valid passports part2: {valid_passport_count2}");
}
