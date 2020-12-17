use std::env;

mod day01_20;
mod day02_20;
mod day03_20;
mod day04_20;
mod day05_20;
mod day06_20;
mod day07_20;
mod day08_20;
mod day09_20;
mod day10_20;
mod day11_20;
mod day12_20;
mod day13_20;
mod day14_20;
mod day15_20;
mod day16_20;
mod day17_20;
mod day18_20;
mod day19_20;
mod day20_20;
mod day21_20;
mod day22_20;
mod day23_20;
mod day24_20;
mod day25_20;
mod passport;
mod tools;

fn main() {
    let days: Vec<(&str, fn(bool, bool) -> (u128, u128, u128), usize)> = vec![
        ("Day 01 of 2020", day01_20::run, 500),
        ("Day 02 of 2020", day02_20::run, 500),
        ("Day 03 of 2020", day03_20::run, 500),
        ("Day 04 of 2020", day04_20::run, 500),
        ("Day 05 of 2020", day05_20::run, 500),
        ("Day 06 of 2020", day06_20::run, 500),
        ("Day 07 of 2020", day07_20::run, 500),
        ("Day 08 of 2020", day08_20::run, 500),
        ("Day 09 of 2020", day09_20::run, 500),
        ("Day 10 of 2020", day10_20::run, 500),
        ("Day 11 of 2020", day11_20::run, 100),
        ("Day 12 of 2020", day12_20::run, 500),
        ("Day 13 of 2020", day13_20::run, 500),
        ("Day 14 of 2020", day14_20::run, 100),
        ("Day 15 of 2020", day15_20::run, 10),
        ("Day 16 of 2020", day16_20::run, 500),
        ("Day 17 of 2020", day17_20::run, 100),
        ("Day 18 of 2020", day18_20::run, 500),
        ("Day 19 of 2020", day19_20::run, 500),
        ("Day 20 of 2020", day20_20::run, 500),
        ("Day 21 of 2020", day21_20::run, 500),
        ("Day 22 of 2020", day22_20::run, 500),
        ("Day 23 of 2020", day23_20::run, 500),
        ("Day 24 of 2020", day24_20::run, 500),
        ("Day 25 of 2020", day25_20::run, 500),
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please specify the day to run!");
        return;
    }

    if let Ok(real) = args[2].parse::<bool>() {
        println!("-----------------------------------------------------------------------------------------------------------------------");
        if args[1] == "performance" {
            println!("Running performance check");
            println!("");
            for (name, day, runs) in days.iter() {
                let mut timings: Vec<(u128, u128, u128)> = vec![];
                for _ in 0..*runs {
                    timings.push(day(real, false));
                }
                let total = timings.iter().fold((0, 0, 0), |acc, val| {
                    (acc.0 + val.0, acc.1 + val.1, acc.2 + val.2)
                });
                let mean = (
                    total.0 as f64 / *runs as f64,
                    total.1 as f64 / *runs as f64,
                    total.2 as f64 / *runs as f64,
                );
                let variance = timings
                    .iter()
                    .map(|(v1, v2, v3)| {
                        let diff = (
                            mean.0 - (*v1 as f64),
                            mean.1 - (*v2 as f64),
                            mean.2 - (*v3 as f64),
                        );

                        (diff.0 * diff.0, diff.1 * diff.1, diff.2 * diff.2)
                    })
                    .fold((0.0, 0.0, 0.0), |acc, val| {
                        (
                            acc.0 + val.0 / (*runs as f64),
                            acc.1 + val.1 / (*runs as f64),
                            acc.2 + val.2 / (*runs as f64),
                        )
                    });

                println!("{} - init: {:>10.4} +/- {:>10.4} ms, p1: {:>10.4} +/- {:>10.4} ms, p2: {:>10.4} +/- {:>10.4} ms", 
                    name, 
                    mean.0 / 1_000_000f64, variance.0.sqrt() / 1_000_000f64, 
                    mean.1 / 1_000_000f64, variance.1.sqrt() / 1_000_000f64, 
                    mean.2 / 1_000_000f64, variance.2.sqrt() / 1_000_000f64
                );
            }
        } else if args[1] == "all" {
            for (name, day, _) in days.iter() {
                let timing = day(real, false);
                println!(
                    "{} - init: {:10} ns, p1: {:10} ns, p2: {:10} ns",
                    name, timing.0, timing.1, timing.2
                );
            }
        } else {
            if let Ok(i) = args[1].parse::<usize>() {
                println!("{}", days[i - 1].0);
                println!("");
                let timing = days[i - 1].1(real, true);
                println!("");
                println!(
                    "Timing - init: {:10} ns, p1: {:10} ns, p2: {:10} ns",
                    timing.0, timing.1, timing.2
                );
            }
        }
        println!("-----------------------------------------------------------------------------------------------------------------------");
    }
}
