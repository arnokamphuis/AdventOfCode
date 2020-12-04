use super::tools;
use regex::Regex;
use std::collections::BTreeMap;
use std::time::Instant;

pub fn valid(passport: &BTreeMap<String, String>, part: usize) -> bool {
    if part == 1 {
        return passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid");
    }

    let mut valid: bool = true;
    if part == 2 {
        if let Some(byr_s) = passport.get("byr") {
            let byr = byr_s.parse::<i32>().unwrap();
            if byr < 1920 || byr > 2002 {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(iyr_s) = passport.get("iyr") {
            let iyr = iyr_s.parse::<i32>().unwrap();
            if iyr < 2010 || iyr > 2020 {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(eyr_s) = passport.get("eyr") {
            let eyr = eyr_s.parse::<i32>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(hgt_s) = passport.get("hgt") {
            let value_s: String = hgt_s.chars().take(hgt_s.len() - 2).collect();
            let unit: String = hgt_s.chars().skip(hgt_s.len() - 2).collect();

            if let Ok(value) = value_s.parse::<i32>() {
                if unit == "in" {
                    if value < 59 || value > 76 {
                        valid = false
                    }
                } else if unit == "cm" {
                    if value < 150 || value > 193 {
                        valid = false
                    }
                } else {
                    valid = false;
                }
            } else {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(hcl_s) = passport.get("hcl") {
            if hcl_s.len() != 7 {
                valid = false;
            }

            let value_s: String = hcl_s.chars().skip(1).collect();

            let re = Regex::new(r"^[a-f0-9]{6}$").unwrap();
            if !re.is_match(&value_s) {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(ecl) = passport.get("ecl") {
            if !(ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth")
            {
                valid = false;
            }
        } else {
            valid = false;
        }

        if let Some(pid_s) = passport.get("pid") {
            if pid_s.len() != 9 {
                valid = false;
            } else if let Ok(pid) = pid_s.parse::<i64>() {
            } else {
                valid = false;
            }
        } else {
            valid = false;
        }
    } else {
        valid = false;
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
