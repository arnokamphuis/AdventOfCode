use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07_21_test.txt"
    } else {
        "./input/day07_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let positions = input[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let fuel_1 = |p: i32, i: i32| -> i32 { (i-p).abs() as i32 };
    let fuel_2 = |p: i32, i: i32| -> i32 { let f = (i-p).abs(); (f*(f+1)/2) as i32 };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut fuel_total: Vec<i32> = vec![0;(max-min+1) as usize];
    for i in min..=max {
        fuel_total[i as usize] = positions.iter().map(|&p| fuel_1(p,i) ).sum::<i32>();
    }
    let res1 = fuel_total.iter().min().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    fuel_total = vec![0;(max-min+1) as usize];
    for i in min..=max {
        fuel_total[i as usize] = positions.iter().map(|&p| fuel_2(p,i) ).sum::<i32>();
    }
    let res2 = fuel_total.iter().min().unwrap();

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
