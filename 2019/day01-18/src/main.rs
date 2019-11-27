use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::time::{Instant};

fn main() {
    println!("Day 1, 2018");

    // let input_file = "./input/input_test.txt";
    let input_file = "./input/input_real.txt";

    let mut current_freq: i64 = 0;
    let mut mutations = Vec::new();

    let start = Instant::now();

    if let Ok(lines) = read_lines(input_file) {
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


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}