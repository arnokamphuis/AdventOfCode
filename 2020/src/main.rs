use std::env;

mod day01_20;
mod day02_20;
mod day03_20;
mod day04_20;
mod day05_20;
mod day06_20;
mod day07_20;
mod passport;
mod tools;

fn main() {

    let days: Vec<(&str, fn(bool, bool) -> (u128, u128, u128))> = vec![
        ("Day 1 of 2020", day01_20::run), 
        ("Day 2 of 2020", day02_20::run),
        ("Day 3 of 2020", day03_20::run), 
        ("Day 4 of 2020", day04_20::run),
        ("Day 5 of 2020", day05_20::run), 
        ("Day 6 of 2020", day06_20::run),
        ("Day 7 of 2020", day07_20::run)
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify the day to run!");
        return
    }

    if let Ok(real) = args[2].parse::<bool>() {

        if args[1] == "all" {
            for (name,day) in days.iter() {
                let timing = day(real, false);
                println!("{} - Timing - init: {:10} ns, p1: {:10} ns, p2: {:10} ns", name, timing.0, timing.1, timing.2);
            }

        } else {
            println!("-----------------------------------------------------------------------------------------------------");
            if let Ok(i) = args[1].parse::<usize>() {
                println!("{}", days[i-1].0);
                println!("");
                let timing = days[i-1].1(real, true);
                println!("");
                println!("Timing - init: {:10} ns, p1: {:10} ns, p2: {:10} ns", timing.0, timing.1, timing.2);
            }
            println!("-----------------------------------------------------------------------------------------------------");
        }
    }
}
