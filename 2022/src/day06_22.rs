use super::tools;
use std::time::Instant;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06_22_test.txt"
    } else {
        "./input/day06_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let processed_lines = input.iter().map(|line| {
        line.chars().map(|c| c as usize).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();

    let different = | x: &[usize] | -> bool {
        x.iter().map(|&v| v).collect::<HashSet<usize>>().len() == x.len()
    };
    
    let find_different = | v: &Vec<usize>, l: usize | -> usize {
        v.windows(l).position(|x| different(x) ).unwrap() + l
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = processed_lines
        .iter()
        .map(|line| { find_different(line,4) })
        .collect::<Vec<_>>()[0];

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = processed_lines
        .iter()
        .map(|line| { find_different(line,14) })
        .collect::<Vec<_>>()[0];

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
