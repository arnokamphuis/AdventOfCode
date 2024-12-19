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

fn count(design: U320, target: &U320, designs: &Vec<U320>, patterns: &Vec<U320>, cache: &mut HashMap<U320, usize>, part: u8) -> usize {
    let key = design;
    if cache.contains_key(&key) {
        return cache[&key];
    }
    
    if design == *target {
        return 1;
    }

    if design > *target {
        return 0;
    }

    let count = patterns
        .iter()
        .map(|pattern| { 
            (design << mult5!(pattern.bits())) | *pattern 
        })
        .filter(|new_design| {
            let target_bits = mult5!(target.bits());
            let new_design_bits = mult5!(new_design.bits());
            let diff_bits = target_bits - new_design_bits;
            *new_design == (target >> diff_bits)
        })
        .filter(|new_design| new_design <= target)
        .fold(0, |acc, new_design| {
            if part == 1 && acc > 0 { acc } else {
                acc + count(new_design, target, designs, patterns, cache, part)
            }
        });

    cache.insert(key, count);
    count
}

fn count_all(designs: &Vec<U320>, patterns: &Vec<U320>, part: u8) -> usize {
    designs
        .iter()
        .map(|design| { count(U320::from(0), &design, designs, patterns, &mut HashMap::new(), part) })
        .map(|count| if part == 1 { if count > 0 { 1 } else { 0 } } else { count })
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
            .map(|ch| ch as u8 - 'a' as u8)
            .fold(U320::from(0), |acc, e| { (acc << 5) | U320::from(e) }))
        .collect::<Vec<U320>>();

    let designs = input[2..]
        .iter()
        .map(|s| s
            .chars()
            .map(|ch| ch as u8 - 'a' as u8 )
            .fold(U320::from(0), |acc, e| { (acc << 5) | U320::from(e) }))
        .collect::<Vec<U320>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = count_all(&designs, &patterns, 1);
    // let res1 = 0;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = count_all(&designs, &patterns, 2);
    // let res2 = 0;

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
