use super::tools;
use std::collections::BTreeMap;
use std::time::Instant;
use super::passport::Passport;
use std::iter::FromIterator;

#[allow(dead_code)]
pub fn run() {
    println!("Day 04 of 2020");

    let start0 = Instant::now();

    // let input_file: &str = "./input/day04_20_test.txt";
    let input_file: &str = "./input/day04_20_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut valid_passports_strict: Vec<Passport> = vec![];
    let mut valid_passports_loose: Vec<Passport> = vec![];

    let mut current_passport: BTreeMap<String, String> = BTreeMap::new();

    for line in &input {
        if line == "" {
            // part 1 check
            if let Ok(valid_pp) = Passport::new(&current_passport, false) {
                valid_passports_loose.push(valid_pp);
            }

            // part 2 check
            if let Ok(valid_pp) = Passport::new(&current_passport, true) {
                valid_passports_strict.push(valid_pp);
            }

            // start new passport
            current_passport = BTreeMap::new();
        } else {
            let items: Vec<&str> = line.split(' ').collect();
            for item in items {
                let keyvaluepair: Vec<&str> = item.split(':').collect();
                current_passport
                    .insert(String::from(keyvaluepair[0]), String::from(keyvaluepair[1]));
            }
        }
    }

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();
    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", valid_passports_loose.len(), after1.duration_since(start1));

    let start2 = Instant::now();
    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", valid_passports_strict.len(), after2.duration_since(start2));

    // let mut cids: BTreeMap<u32, u32> = BTreeMap::new();
    // for pp in valid_passports_strict {
    //     if pp.cid_num >= 0 {
    //         let cid = cids.entry(pp.cid_num as u32).or_insert(0);
    //         *cid += 1
    //     }
    // }

    // let mut v = Vec::from_iter(cids);
    // v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    // println!("{:?}", v);
}
