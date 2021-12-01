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

    let after0 = Instant::now();

    let start1 = Instant::now();

    let numbers: Vec<i64> = (&input)
        .into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let res1 = numbers
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, n1)| *n1 > &numbers[i - 1])
        .count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = numbers
        .iter()
        .enumerate()
        .skip(2)
        .map(|(i, n)| n + numbers[i - 1] + numbers[i - 2])
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|n| n[1] > n[0])
        .count();

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
