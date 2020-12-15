use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;

pub fn get_number(numbers: &Vec<u32>, it: u32) -> u32 {
    let mut memory: BTreeMap<u32,u32> = BTreeMap::new();
    let mut last: u32;

    last = numbers[0];
    for (i, &n) in numbers.iter().skip(1).enumerate() {
        memory.insert(last,i as u32 + 2);
        last = n;
    }

    for count in numbers.len() as u32..it {
        let next;
        let turn = count + 1;
        if let Some(time) = memory.get(&last) {
            next = turn - time;
        } else {
            next = 0;
        }

        if let Some(prev) = memory.get_mut(&last) {
            *prev = turn;
        } else {
            memory.insert(last, turn);
        }

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
    let numbers: Vec<u32> = input[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = get_number(&numbers, 2020);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = get_number(&numbers, 30000000);

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
