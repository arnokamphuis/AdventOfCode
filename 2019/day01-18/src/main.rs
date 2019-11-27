use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    println!("Day 1, 2018");

    // let input_file = "./input/input_test.txt";
    let input_file = "./input/input_real.txt";

    let mut current_freq: i64 = 0;
    let mut mutations = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                let delta = value.parse::<i64>().unwrap();
                mutations.push(delta);
                current_freq += delta;
            }
        }
    }
    println!("Part 1: {}", current_freq);

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

    println!("Part 2: {}", current_freq);

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}