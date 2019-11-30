use std::time::{Instant};
use super::tools;

pub fn run() {
    println!("Day 1 of 2019");

    let input_file = "./input/day01_19_test.txt";
    // let input_file = "./input/day01_19_real.txt";

    let start1 = Instant::now();

    let input = tools::get_input(String::from(input_file));

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", 0, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}
