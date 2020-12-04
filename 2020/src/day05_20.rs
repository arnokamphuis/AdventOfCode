use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run() {
    println!("Day 05 of 2020");

    let start0 = Instant::now();

    let input_file: &str = "./input/day05_20_test.txt";
    // let input_file: &str = "./input/day05_20_real.txt";
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", 0, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}
