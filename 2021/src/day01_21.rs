use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day01_21_test.txt"
    } else {
        "./input/day01_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let numbers: Vec<i64> = (&input)
        .into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = numbers.windows(2).filter(|v| v[1] > v[0]).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = numbers.windows(4).filter(|v| v[3] > v[0]).count();

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
