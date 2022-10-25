use super::tools;
use std::time::Instant;
use itertools::sorted;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day02_15_test.txt"
    } else {
        "./input/day02_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let packages: Vec<Vec<i32>> = input.iter().map(|line| {
        sorted(line.split("x").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<i32>>()
    }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: i32 = packages.iter()
        .map(|p| 3 * p[0]*p[1] + 2 * (p[1]*p[2] + p[0]*p[2]))
        .sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: i32 = packages.iter()
        .map(|p| 2 * (p[0] + p[1]) + p[0]*p[1]*p[2])
        .sum();

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
