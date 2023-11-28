use super::tools;
use std::time::Instant;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day03_22_test.txt"
    } else {
        "./input/day03_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    // a rucksacks consists of (compartment1, compartment2, fullsack)
    let rucksacks: Vec<(HashSet<char>,HashSet<char>,HashSet<char>)> = input.iter().map(|line| {
        (
            line[0..line.len()/2]
                .chars()
                .collect::<HashSet<char>>(),
            line[line.len()/2..]
                .chars()
                .collect::<HashSet<char>>(),
            line.chars().collect::<HashSet<char>>()
        )
    }).collect();


    let intersect = | sets: Vec<&HashSet<char>> | -> char {
        *sets
            .iter()
            .skip(1)
            .fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            })
            .iter()
            .next()
            .unwrap()
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let value = | c: char | -> usize {
        match c {
            'A'..='Z' => c as usize - 'A' as usize + 27,
            _         => c as usize - 'a' as usize +  1
        }
    };

    let res1: usize = rucksacks
        .iter()
        .map(|(comp1, comp2, _)| intersect(vec![comp1, comp2]))
        .map(|c| value(c))
        .sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = rucksacks
        .chunks(3)
        .map(|sacks| intersect(vec![&sacks[0].2, &sacks[1].2, &sacks[2].2]))
        .map(|c| value(c))
        .sum();

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
