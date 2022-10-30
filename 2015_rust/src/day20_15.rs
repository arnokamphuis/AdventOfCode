use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day20_15_test.txt"
    } else {
        "./input/day20_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let target = input[0].parse::<usize>().unwrap();

    let divisors = | v: usize | -> Vec<usize> {
        let s = (v as f64).sqrt() as usize;
        (1..=s).filter(|n| v % n == 0).collect()
    };

    let house_score = | h: usize | -> usize {
        divisors(h).iter().map(|d| 10 * (h/d + d)).sum()
    };

    let new_house_score = | h: usize | -> usize {
        divisors(h).iter().map(|&d| {
            let od = h/d;
            11 * (if od <= 50 { d } else { 0 } + if d <= 50 { od } else { 0 })
        }).sum()
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut house: usize = 1;
    while house_score(house) < target { 
        house += 1; 
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", house);
    }

    let start2 = Instant::now();

    let mut house: usize = 1;
    while new_house_score(house) < target { 
        house += 1; 
    }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", house);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
