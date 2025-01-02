use super::tools;
use std::time::Instant;
use regex::Regex;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day03-test.txt"
    } else {
        "./input/day03-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let full_input = input.join(" ");


    let after0 = Instant::now();

    let start1 = Instant::now();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let res1 = re
        .captures_iter(&full_input)
        .map(|cap| {
            cap[0][4..cap[0].len()-1].split(',').map(|x| x.parse::<u32>().unwrap()).fold(1, |acc, x| acc * x)
        })
        .sum::<u32>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    let (_,res2) = re
        .captures_iter(&full_input)
        .fold((true, 0), |acc, cap| {
            match &cap[0] {
                "do()" => (true, acc.1),
                "don't()" => (false, acc.1),
                _ => {
                    (acc.0, 
                        acc.1 + acc.0 as u32 * 
                            cap[0][4..cap[0].len()-1]
                                .split(',')
                                .map(|x| x.parse::<u32>()
                                .unwrap())
                                .fold(1, |acc, x| acc * x))
                }
            }
        });


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
