use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

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
    let rucksacks: Vec<(BTreeSet<char>,BTreeSet<char>,BTreeSet<char>)> = input.iter().map(|line| {
        (
            line[0..line.len()/2]
                .chars()
                .collect::<BTreeSet<char>>(),
            line[line.len()/2..]
                .chars()
                .collect::<BTreeSet<char>>(),
            line.chars().collect::<BTreeSet<char>>()
        )
    }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let value = | c: char | -> usize {
        let v = c as usize - 65;
        if v > 26 { v - 31 } else { v + 27 } 
    };

    let res1: usize = rucksacks
        .iter()
        .map(|(comp1, comp2, _)| {
            value(
                comp1
                    .intersection(&comp2)
                    .cloned()
                    .collect::<Vec<char>>()[0]
                )})
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = rucksacks
        .chunks(3)
        .map(|sacks| 
            value(
                sacks[0].2
                    .intersection(&sacks[1].2)
                    .cloned()
                    .collect::<BTreeSet<char>>()
                    .intersection(&sacks[2].2)
                    .cloned()
                    .collect::<Vec<char>>()[0]
                ))
        .collect::<Vec<usize>>()
        .iter()
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
