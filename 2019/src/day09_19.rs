use super::tools;
use std::time::Instant;
// use std::collections::HashMap;
use std::collections::BTreeMap;

use super::intcode::IntCodeComputer;

#[allow(dead_code)]
pub fn run() {
    println!("Day 9 of 2019");

    // let input_file = "./input/day09_19_test.txt";
    let input_file = "./input/day09_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let mut commands: BTreeMap<i64, i64> = BTreeMap::new();
    command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .for_each(|(i, c)| {
            commands.insert(i as i64, c);
        });

    let start1 = Instant::now();

    let mut computer1 = IntCodeComputer::new(&commands);
    computer1.add_input(1);
    computer1.run();

    let after1 = Instant::now();
    println!(
        "Part 1: {:?}, in {:?}",
        computer1.get_output().unwrap(),
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut computer2 = IntCodeComputer::new(&commands);
    computer2.add_input(2);
    computer2.run();

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        computer2.get_output().unwrap(),
        after2.duration_since(start2)
    );
}
