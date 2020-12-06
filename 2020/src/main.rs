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

    let days = [
        day01_20::run, 
        day02_20::run,
        day03_20::run, 
        day04_20::run,
        day05_20::run, 
        day06_20::run
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify the day to run!");
        return
    }

    if let Ok(real) = args[2].parse::<bool>() {

        if args[1] == "all" {
            for day in days.iter() {
                println!("");
                day(real);
            }

        } else {
            match args[1].parse::<usize>() {
                Ok(i) => days[i-1](real),
                _ => println!("Day {} is not available.", args[1])
            }
        }
    }
}
