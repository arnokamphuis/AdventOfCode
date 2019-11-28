use std::collections::HashSet;
use std::time::{Instant};

use super::tools;

pub fn run() {
    println!("Day 1, 2018");

    // let input_file = "./input/input_test.txt";
    let input_file = "./input/day01_18_real.txt";

    let mut current_freq: i64 = 0;
    let mut mutations = Vec::new();

    let start = Instant::now();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                let delta = value.parse::<i64>().unwrap();
                mutations.push(delta);
                current_freq += delta;
            }
        }
    }

    let after_part1 = Instant::now();
    println!("Part 1: {}, in {:?}", current_freq, after_part1.duration_since(start));

    let mut frequencies = HashSet::new();
    current_freq = 0;
    let mut current = 0;

    frequencies.insert(current_freq);
    loop {
        current_freq += mutations[current % mutations.len()];
        if frequencies.contains(&current_freq){
            break;
        } else {
            frequencies.insert(current_freq);
        }
        current += 1;
    }
    let after_part2 = Instant::now();

    println!("Part 2: {}, in {:?}", current_freq, after_part2.duration_since(after_part1));

}

