use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day01_15_test.txt"
    } else {
        "./input/day01_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = input[0]
        .chars()
        .fold(0, |floor, c| { match c { '(' => floor + 1, ')' => floor - 1, _ => floor } });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = input[0]
        .chars()
        .map(|c| { match c { '(' => 1, ')' => -1, _ => 0 } })
        .fold(vec![0], |mut f, d| { let n = f.last().unwrap() + d; f.push(n); f })
        .iter()
        .position(|&x| x < 0)
        .unwrap();

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
