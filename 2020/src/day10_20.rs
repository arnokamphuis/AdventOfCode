use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10_20_test.txt"
    } else {
        "./input/day10_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut numbers: Vec<i64> = input.iter().map(|v| v.parse::<i64>().unwrap()).collect();
    numbers.push(numbers.iter().max().unwrap()+3);
    numbers.push(0);
    numbers.sort();


    let after0 = Instant::now();

    let start1 = Instant::now();

    let gaps = numbers
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index,_)| numbers[index] - numbers[index-1] )
        .collect::<Vec<i64>>();

    let res1 = gaps.iter().filter(|&&v| v == 1).count() * gaps.iter().filter(|&&v| v == 3).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut count: Vec<u128> = vec![0; numbers.len()];
    count[0] = 1;

    for index in 0..numbers.len()-1 {
        let current_count = count[index];
        let current_jolt = numbers[index]; 

        for i2 in 1..numbers.len()-index {
            if numbers[index+i2]>current_jolt+3 {
                break;
            }
            for delta in 0..4i64 {
                if numbers[index+i2]==current_jolt+delta {
                    count[index+i2] += current_count;
                }
            }
        }
    }

    let res2 = count[numbers.len()-1];

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
