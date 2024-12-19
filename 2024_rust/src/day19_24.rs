use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct VecKey(Vec<u8>);

impl Hash for VecKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_slice().hash(state);
    }
}

impl PartialEq for VecKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for VecKey {}

fn count(design: Vec<u8>, target: &Vec<u8>, designs: &Vec<Vec<u8>>, patterns: &Vec<Vec<u8>>, cache: &mut HashMap<VecKey, usize>) -> usize {
    let key = VecKey(design.clone());
    if cache.contains_key(&key) {
        return cache[&key];
    }
    
    if design == *target {
        return 1;
    }

    if design.len() > target.len() {
        return 0;
    }

    let count = patterns
        .iter()
        .map(|pattern| { pattern.iter().fold(design.clone(), |mut acc, e| { acc.push(*e); acc })})
        .filter(|new_design| new_design.len() <= target.len())
        .filter(|new_design| *new_design == target[..new_design.len()])
        .fold(0, |acc, new_design| {
            acc + count(new_design, target, designs, patterns, cache)
        });

    cache.insert(key, count);
    count
}

fn count_all(designs: &Vec<Vec<u8>>, patterns: &Vec<Vec<u8>>, part: u8) -> usize {
    designs
        .iter()
        .map(|design| { count(vec![], &design, designs, patterns, &mut HashMap::new()) })
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
    let patterns = input[0]
        .split(", ")
        .map(|s| s
            .chars()
            .map(|ch| ch as u8)
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let designs = input[2..]
        .iter()
        .map(|s| s.chars().map(|ch| ch as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

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
