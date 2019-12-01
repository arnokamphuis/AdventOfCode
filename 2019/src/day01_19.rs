use std::time::{Instant};
use super::tools;
use math::round;

fn calculate_fuel(mass: i64) -> i64 {
    let fuel_for_mass = round::floor(mass as f64 / 3.0, 0) as i64 - 2;
    if fuel_for_mass <= 0 { 0 as i64 } else { fuel_for_mass + calculate_fuel(fuel_for_mass) }
}

pub fn run() {
    println!("Day 1 of 2019");

    // let input_file = "./input/day01_19_test.txt";
    let input_file = "./input/day01_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let start1 = Instant::now();

    let mut fuel: i64 = input.clone().into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .map(|mass| round::floor(mass as f64 / 3.0, 0) as i64 - 2)
        .sum::<i64>();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", fuel, after1.duration_since(start1));

    let start2 = Instant::now();

    fuel = input.clone().into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .map(|mass| calculate_fuel(mass))
        .sum::<i64>();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", fuel, after2.duration_since(start2));
}
