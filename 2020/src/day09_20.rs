use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_20_test.txt"
    } else {
        "./input/day09_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let window: usize = if real { 25 } else {5};
    let numbers: Vec<i64> = input.iter().map(|v| v.parse::<i64>().unwrap()).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut contiguoustarget: i64 = 0;

    for i in window..numbers.len() {
        let target = numbers.iter().nth(i).unwrap();
        let sources = numbers.iter().skip(i-window).take(window).collect::<Vec<&i64>>();

        let mut found = false;
        for j in 0..sources.len() {
            for k in 0..sources.len() {
                if j!=k {
                    if sources[j] + sources[k] == *target {
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            contiguoustarget = numbers[i];
            break;
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", contiguoustarget);
    }

    let start2 = Instant::now();

    let mut ans2 = 0;
    for i in 0..numbers.len()-2 {
        let mut sum: i64 = numbers[i] + numbers[i+1];
        for j in i+2..numbers.len() {
            sum += numbers[j];
            if sum == contiguoustarget {
                let set = numbers.iter().skip(i).take(j-i).map(|&v| v).collect::<Vec<i64>>();
                ans2 = set.iter().min().unwrap() + set.iter().max().unwrap()
            }
            if sum > contiguoustarget {
                break;
            }
        }
        if ans2 > 0 {
            break;
        }
    }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", ans2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
