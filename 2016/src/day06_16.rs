use std::collections::HashMap;
use std::time::{Instant};

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 6 of 2016");

    // let input_file = "./input/day06_16_test.txt";
    let input_file = "./input/day06_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut thefrequencies = Vec::new();
    for _ in 0..input[0].len() {
        let frequency: HashMap<char, u32> = HashMap::new();
        thefrequencies.push(frequency);
    }

    for word in &input {
        let mut count = 0;
        for c in word.chars() {
            *thefrequencies[count].entry(c).or_insert(0) += 1;
            count += 1;
        }
    }

    let mut code = String::from("");
    let mut modcode = String::from("");
    for map in thefrequencies {
        let mut count_vec: Vec<(&char, &u32)> = map.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let top = &count_vec[0];
        code.push(*top.0);
        count_vec.sort_by(|b, a| b.1.cmp(a.1));
        let top = &count_vec[0];
        modcode.push(*top.0);
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", code, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", modcode, after2.duration_since(start2));
}
