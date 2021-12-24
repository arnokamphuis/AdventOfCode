use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/dayxx_21_test.txt"
    } else {
        "./input/dayxx_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();

    let start1 = Instant::now();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", 0);
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
