use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_15_test.txt"
    } else {
        "./input/day08_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: usize = input.iter().map(|line| {
        let mut count = 0usize;
        let mut chars = line.chars();
        chars.next();
        chars.next_back();
        while let Some(c) = chars.next() {
            if c == '\\' {
                count += 1;
                if let Some(nc) = chars.next() {
                    if !(nc == '\\' || nc == '\"') {
                        chars.next();
                        chars.next();
                    }
                } else {
                    panic!("should not be here");
                }
            } else {
                count += 1;
            }
        }
        line.len() - count
    }).sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = input.iter().map(|line| {
        line.chars().fold(2, |t, c| t + (c == '\\' || c == '"') as usize )
    }).sum();

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
