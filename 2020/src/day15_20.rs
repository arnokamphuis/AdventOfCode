use super::tools;
use std::time::Instant;

#[allow(unused_must_use)]
pub fn get_number(numbers: &Vec<usize>, it: usize) -> usize {
    let mut memory = vec![0; it];
    let mut last: usize;

    last = numbers[0];
    for (i, &n) in numbers.iter().skip(1).enumerate() {
        memory[last] = i + 2;
        last = n;
    }

    let start: usize = numbers.len() + 1;
    let end: usize = it+1;
    for turn in start..end {
        let next: usize = match memory[last] {
            0 => 0,
            lt => turn - lt
        };
        memory[last] = turn;
        last = next;
    }

    last
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15_20_test.txt"
    } else {
        "./input/day15_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let numbers: Vec<usize> = input[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = get_number(&numbers, 2020);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = get_number(&numbers, 30_000_000);

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
