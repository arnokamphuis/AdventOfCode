use std::time::{Instant};
use super::tools;
use math::round;

fn calculate_fuel(mass: i64, extra: bool) -> i64 {
    let fuel_for_mass = round::floor(mass as f64 / 3.0, 0) as i64 - 2;
    if fuel_for_mass <= 0 { 
        0 as i64 
    } else {
        let mut fuel = fuel_for_mass;
        if extra {
            if fuel > 0 {
                fuel += calculate_fuel(fuel, extra);
            }
        }
        fuel
    }
}

pub fn run() {
    println!("Day 1 of 2019");

    // let input_file = "./input/day01_19_test.txt";
    let input_file = "./input/day01_19_real.txt";

    let start1 = Instant::now();

    let input = tools::get_input(String::from(input_file));

    let mut fuel: i64 = 0;
    for line in &input {
        let mass = line.parse::<i64>().unwrap();
        let fuel_for_mass = calculate_fuel(mass,false);
        fuel += fuel_for_mass;
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", fuel, after1.duration_since(start1));

    let start2 = Instant::now();

    fuel = 0;
    for line in &input {
        let mass = line.parse::<i64>().unwrap();
        let fuel_for_mass = calculate_fuel(mass,true);
        fuel += fuel_for_mass;
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", fuel, after2.duration_since(start2));
}
