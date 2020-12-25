use super::tools;
use std::time::Instant;

fn find_loopsize(subject_number: u64, target: u64) -> usize {
    let mut value = subject_number;
    let mut loopsize = 0;
    while value != target {
        value = ( value * subject_number ) % 20201227u64; 
        loopsize += 1;
    }
    loopsize

}

fn encrypt(subject_number: u64, loopsize: usize) -> u64 {
    let mut value = subject_number;
    for _ in 0..loopsize {
        value = ( value * subject_number ) % 20201227u64; 
    }
    value
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day25_20_test.txt"
    } else {
        "./input/day25_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let c_pid = input[0].parse::<u64>().unwrap();
    let d_pid = input[1].parse::<u64>().unwrap();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let c_loopsize = find_loopsize(7, c_pid);
    let key = encrypt(d_pid, c_loopsize);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", key);
    }

    let start2 = Instant::now();

    let res2 = "Yes! Fifty stars :-) ";

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
