use super::tools;
use std::time::Instant;
use itertools::sorted;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day01_22_test.txt"
    } else {
        "./input/day01_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut elfs: Vec<Vec<usize>> = vec![];
    let mut current_elf: Vec<usize> = vec![];
    for line in &input {
        if line.len() == 0 {
            elfs.push(current_elf.clone());
            current_elf = vec![];
        } else {
            current_elf.push(line.parse::<usize>().unwrap());
        }
    }
    elfs.push(current_elf.clone());

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = elfs.iter().map(|v| v.iter().sum::<usize>()).max().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = sorted(
        elfs
            .iter()
            .map(|v| v.iter().sum::<usize>())
            .collect::<Vec<usize>>()
        ).collect::<Vec<usize>>()
        .iter()
        .rev()
        .take(3)
        .sum::<usize>();

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
