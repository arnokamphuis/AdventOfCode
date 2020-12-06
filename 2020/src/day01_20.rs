use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool) {
    println!("Day 1 of 2020");

    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day01_20_test.txt" } else { "./input/day01_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let numbers: Vec<i64> = (&input).into_iter().map(|n| n.parse::<i64>().unwrap()).collect();

    let mut res = 0;
    for item1 in &numbers {
        for item2 in &numbers {
            if item1+item2==2020 {
                res = item1*item2;
            }

        }
    }

    let after1 = Instant::now();
    println!("Part 1: {} in {:?}", res, after1.duration_since(start1));

    let start2 = Instant::now();

    res = 0;
    for item1 in &numbers {
        for item2 in &numbers {
            for item3 in &numbers {
                if item1+item2+item3==2020 {
                    res = item1*item2*item3;
                }
            }
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {} in {:?}", res, after2.duration_since(start2));
}
