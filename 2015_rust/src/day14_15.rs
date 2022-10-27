use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_15_test.txt"
    } else {
        "./input/day14_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut reindeers: Vec<(String, u64, u64, u64)> = vec![];
    for line in &input {
        let words = line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let name = words[0].to_string();
        let speed = words[3].parse::<u64>().unwrap();
        let duration = words[6].parse::<u64>().unwrap();
        let sleep = words[13].parse::<u64>().unwrap();
        reindeers.push((name.to_string(), speed, duration, sleep));
    }

    let calculate_distance_traveled = | time: u64, speed: u64, duration: u64, sleep: u64 | -> u64 {
        let repeats = time / (duration + sleep);
        let end_of_repeats = (duration + sleep) * repeats;
        let delta_time = duration.min(time - end_of_repeats);
        (repeats * duration + delta_time) * speed
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: u64 = reindeers.iter().map(|r| calculate_distance_traveled(2503, r.1, r.2, r.3) ).max().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut wins = reindeers.iter().map(|(n,_,_,_)| (n.to_string(), 0)).collect::<HashMap<String, u64>>();

    for t in 1..=2503 {
        let locations = reindeers.iter().map(|r| (r.0.to_string(), calculate_distance_traveled(t, r.1, r.2, r.3)) ).collect::<HashMap<String,u64>>();
        let max_loc = locations.iter().map(|(_,&d)| d).max().unwrap();
        let winners = locations.iter().filter(|(_,&d)| d == max_loc ).map(|(n,_)| n.to_string()).collect::<Vec<String>>();
        for w in &winners {
            *wins.get_mut(w).unwrap() += 1;
        }
    }
    let res2 = wins.iter().map(|(_,&w)| w).max().unwrap();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
