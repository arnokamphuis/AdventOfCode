use super::tools;
use std::time::Instant;

fn check(numbers: &Vec<i32>) -> bool {

    let same_sign = numbers
        .windows(2)
        .map(|pair| {
            (pair[0] - pair[1]).signum()
        })
        .try_fold(None, |acc, sign| {
            match acc {
                None => Ok(Some(sign)),
                Some(prev) if prev == sign => Ok(acc),
                _ => Err(()),
            }
        }).is_ok();

    let max_dist = numbers
        .windows(2)
        .map(|pair| {
            (pair[0] - pair[1]).abs()
        })
        .all(|dist| dist > 0 && dist <= 3);
    return same_sign && max_dist;
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day02-test.txt"
    } else {
        "./input/day02-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let lines = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();
    
    let after0 = Instant::now();

    let start1 = Instant::now();
    
    let res1 = lines
        .iter()
        .map(|numbers| check(numbers))
        .filter(|x| *x)
        .count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = lines
        .iter()
        .map(|numbers| {
            (0..numbers.len())
                .map(|i| { check(&[&numbers[..i], &numbers[i + 1..]].concat()) })
                .filter(|x| *x)
                .count() > 0
        })
        .filter(|x| *x)
        .count();

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
