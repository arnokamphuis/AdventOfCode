use super::tools;
use regex::Regex;
use std::collections::BTreeMap;
use std::time::Instant;

pub fn valid(passport: &BTreeMap<String, String>, part: usize) -> bool {
    let mut valid: bool = true;

    valid &= passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid");

    if part == 2 {

        if let Some(byr_s) = passport.get("byr") {
            let byr = byr_s.parse::<i32>().unwrap();
            valid &= byr >= 1920 && byr <= 2002;
        }

        if let Some(iyr_s) = passport.get("iyr") {
            let iyr = iyr_s.parse::<i32>().unwrap();
            valid &= iyr >= 2010 && iyr <= 2020;
        }

        if let Some(eyr_s) = passport.get("eyr") {
            let eyr = eyr_s.parse::<i32>().unwrap();
            valid &= eyr >= 2020 && eyr <= 2030;
        }

        if let Some(hgt_s) = passport.get("hgt") {
            let value_s: String = hgt_s.chars().take(hgt_s.len() - 2).collect();
            let unit: String = hgt_s.chars().skip(hgt_s.len() - 2).collect();

            if let Ok(value) = value_s.parse::<i32>() {
                if unit == "in" {
                    valid &= value >= 59 && value <= 76;
                } else if unit == "cm" {
                    valid &= value >= 150 && value <= 193;
                } else {
                    valid = false;
                }
            }
        }

        if let Some(hcl_s) = passport.get("hcl") {
            let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            valid &= re.is_match(&hcl_s) && hcl_s.len()==7;
        }

        if let Some(ecl) = passport.get("ecl") {
            valid &= ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth";
        }

        if let Some(pid_s) = passport.get("pid") {
            valid &= pid_s.len() == 9;

            if let Err(_) = pid_s.parse::<i64>() {
                valid = false;
            }
        }

    }

    valid
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 04 of 2020");

    let start0 = Instant::now();

    // let input_file: &str = "./input/day04_20_test.txt";
    let input_file: &str = "./input/day04_20_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut passports: Vec<BTreeMap<String, String>> = vec![];
    let mut current_password: BTreeMap<String, String> = BTreeMap::new();

    for line in &input {
        if line == "" {
            passports.push(current_password);
            current_password = BTreeMap::new();
        } else {
            let items: Vec<&str> = line.split(' ').collect();
            for item in items {
                let keyvaluepair: Vec<&str> = item.split(':').collect();
                current_password
                    .insert(String::from(keyvaluepair[0]), String::from(keyvaluepair[1]));
            }
        }
    }
    passports.push(current_password);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut res1 = 0;

    for pp in &passports {
        let valid = valid(&pp, 1);
        if valid {
            res1 += 1;
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut res2 = 0;

    for pp in &passports {
        let valid = valid(&pp, 2);
        if valid {
            res2 += 1;
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}
