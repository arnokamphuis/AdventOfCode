use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day03_22_test.txt"
    } else {
        "./input/day03_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let value = | c: char | -> usize {
        match c {
            'A'..='Z' => c as usize - 'A' as usize + 26,
            _         => c as usize - 'a' as usize +  0
        }
    };

    let score = | v: u64 | -> usize {
        1 + (0..52).fold(0, |acc, i| { if (v >> i) == 1 { i } else { acc } })
    };

    let rucksacks: Vec<(u64,u64)> = input.iter().map(|line| {
        (
            line[0..line.len()/2]
                .chars()
                .fold(0, |acc, c| acc | 1u64 << value(c)),
            line[line.len()/2..]
                .chars()
                .fold(0, |acc, c| acc | 1u64 << value(c))
        )
    }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: usize = rucksacks
        .iter()
        .map(|(comp1, comp2)| score(comp1 & comp2) )
        .sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2: usize = rucksacks
        .chunks(3)
        .map(|sacks| 
            sacks
                .iter()
                .map(|sack| sack.0 | sack.1)
                .fold(u64::MAX, | acc, sack | acc & sack )
            )
        .map(|c| score(c))
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
