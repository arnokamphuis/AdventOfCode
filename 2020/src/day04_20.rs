use super::tools;
use std::time::Instant;
use super::passport::load_passports;

#[allow(dead_code)]
pub fn run(real: bool) {
    println!("Day 04 of 2020");

    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day04_20_test.txt" } else { "./input/day04_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let valid_passports_loose = load_passports(&input, false);

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", valid_passports_loose.len(), after1.duration_since(start1));

    let start2 = Instant::now();

    let valid_passports_strict = load_passports(&input, true);

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", valid_passports_strict.len(), after2.duration_since(start2));
}
