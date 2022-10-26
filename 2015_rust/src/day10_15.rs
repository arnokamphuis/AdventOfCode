use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10_15_test.txt"
    } else {
        "./input/day10_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut digits = input[0].chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

    let look_and_say = | digs: &Vec<u8> | -> Vec<u8> {
        let mut new_digs: Vec<u8> = vec![];

        let mut index = 0usize;
        while index < digs.len() {
            let mut s = index;
            while s < digs.len() && digs[s]==digs[index] { s+=1; }
            let len = s-index;
            new_digs.push(len as u8); new_digs.push(digs[index]);
            index = s;
        }

        new_digs
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    for _ in 0..40 {
        digits = look_and_say(&digits);
    }
    let res1 = digits.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for _ in 0..10 {
        digits = look_and_say(&digits);
    }
    let res2 = digits.len();

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
