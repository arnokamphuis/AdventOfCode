use std::env;

mod day01_20;
mod day02_20;
mod day03_20;
mod day04_20;
mod day05_20;
mod day06_20;
mod passport;
mod tools;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please specify the day to run!");
        return
    }

    match args[1].parse::<usize>() {
        Ok(1) => day01_20::run(),
        Ok(2) => day02_20::run(),
        Ok(3) => day03_20::run(),
        Ok(4) => day04_20::run(),
        Ok(5) => day05_20::run(),
        Ok(6) => day06_20::run(),
        _ => println!("Day {} is not available.", args[1])
    }
}
