use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06_21_test.txt"
    } else {
        "./input/day06_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut countdowns = input[0]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .fold(vec![0;9], |mut cd, n| { cd[n] += 1; cd});

    let day = |cd: &mut Vec<usize> | {
        let zerocount = cd[0];
        for k in 1..=8 {
            cd[k - 1] = cd[k];
        }
        cd[6] += zerocount;
        cd[8] = zerocount;
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    for _ in 0..80 {
        day(&mut countdowns);
    }

    let res1: usize = countdowns.iter().sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for _ in 80..256 {
        day(&mut countdowns);
    }

    let res2: usize = countdowns.iter().sum();

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
