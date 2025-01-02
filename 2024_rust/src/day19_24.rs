use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use super::construct_uint;

construct_uint! {
    /// 320-bit unsigned integer.
    pub struct U320(5);
}

macro_rules! mult5 {
    ($x:expr) => {
        (($x + 4) / 5) * 5
    };
}

fn check(design: U320, patterns: &Vec<U320>) -> bool {
    if design == U320::from(0) { return true; }
    patterns
        .iter()
        .any(|pattern| {
            let shift_size = mult5!(pattern.bits());
            let mask = U320::from((1 << shift_size)-1);
            if design & mask == *pattern {
                check(design >> shift_size, patterns)
            } else {
                false
            }
        })
}

fn count_fast(design: U320, patterns: &Vec<U320>, cache: &mut HashMap<U320, usize>) -> usize {
    if design == U320::from(0) { return 1; }

    let key = design;
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let res = patterns
        .iter()
        .map(|pattern| {
            let shift_size = mult5!(pattern.bits());
            let mask = (U320::from(1) << shift_size) - 1;
            if design & mask == *pattern {
                let c = count_fast(design >> shift_size, patterns, cache);
                c
            } else {
                0
            }
        })
        .sum();
    cache.insert(key, res);
    res
}

fn check_all(designs: &Vec<U320>, patterns: &Vec<U320>) -> usize {
    designs
        .iter()
        .map(|design| { if check(*design, patterns) { 1 } else { 0 } })
        .sum()
}

fn count_all(designs: &Vec<U320>, patterns: &Vec<U320>) -> usize {
    designs
        .iter()
        .map(|design| { count_fast(*design, patterns, &mut HashMap::new()) })
        .sum()
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day19-test.txt"
    } else {
        "./input/day19-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let patterns: Vec<U320> = input[0]
        .split(", ")
        .map(|s| s
            .chars()
            .map(|ch| ch as u8 - 'a' as u8 + 1)
            .fold(U320::from(0), |acc, e| { 
                (acc << 5) | U320::from(e) 
            }))
        .collect::<Vec<U320>>();

    let designs = input[2..]
        .iter()
        .map(|s| s
            .chars()
            .map(|ch| ch as u8 - 'a' as u8 + 1 )
            .fold(U320::from(0), |acc, e| { (acc << 5) | U320::from(e) }))
        .collect::<Vec<U320>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = check_all(&designs, &patterns);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = count_all(&designs, &patterns);

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
