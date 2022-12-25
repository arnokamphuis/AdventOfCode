use super::tools;
use std::time::Instant;

fn max_value(five_power: u32) -> i64 {
    if five_power == 0 { 
        2_i64
    } else { 
        5_i64.pow(five_power) * 2_i64 + max_value(five_power-1) 
    }
}

fn to_char(d: i64) -> char {
    match d {
        -2 => '=',
        -1 => '-',
         0 => '0',
         1 => '1',
         2 => '2',
         _ => panic!("invalid digit"),
    }
}

fn to_snafu(val: i64, five_power: u32) -> String {
    if val >= -2 && val <= 2 {
        return String::from(to_char(val));
    }

    for d in [-2,-1,0,1,2] {
        let rest_val = val - d * 5_i64.pow(five_power);
        if rest_val.abs() <= max_value( five_power - 1 ) {
            return format!("{}{}", 
                to_char(d), 
                to_snafu(rest_val, five_power - 1)
            ).to_string();
        }
    }

    panic!("should not reach this")
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day25_22_test.txt"
    } else {
        "./input/day25_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let total: i64 = input.iter().map(|line| {
        line.chars().rev().enumerate().fold(0, |acc, (i,v)| {
            let digit: i64 = match v {
                '0' | '1' | '2' => v.to_digit(10).unwrap() as i64,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            acc + 5_i64.pow(i as u32) as i64 * digit
        })
    }).sum();

    let mut five_power: u32 = 0;
    while total.abs() > max_value(five_power) { five_power += 1; }
    let res1 = to_snafu(total, five_power);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
