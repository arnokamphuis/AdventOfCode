use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day25-test.txt"
    } else {
        "./input/day25-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let codes = input.chunks(8).map(|window| {
        (window[0].chars().nth(0).unwrap() == '#',
            window.iter().fold(vec![vec![];window[0].len()], |mut v, line| {
                line.chars().enumerate().for_each(|(col, c)| {
                    if c == '#' { v[col].push(1) } else { v[col].push(0) }
                });
                v
            })
            .iter()
            .map(|v| v.iter().sum::<u8>() - 1)
            .collect::<Vec<u8>>()
        )
    }).collect::<Vec<(bool,Vec<u8>)>>();

    let keys = codes.iter().filter(|(b, _)| *b).map(|(_, v)| v.clone()).collect::<Vec<Vec<u8>>>();
    let locks = codes.iter().filter(|(b, _)| !*b).map(|(_, v)| v.clone()).collect::<Vec<Vec<u8>>>();

    let after0 = Instant::now();

    let start1 = Instant::now();
    let mut res1 = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock
                .iter()
                .zip(key.iter())
                .map(|(c1, c2)| { c1 + c2 })
                .all(|v| v <= 5) {
                    res1 += 1;
                }
        }
    }

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
