use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day04_22_test.txt"
    } else {
        "./input/day04_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let ranges = input
        .iter()
        .map(|line| {
            line
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    let contained = | first: &Vec<usize>, second: &Vec<usize> | -> bool {
        (first[0] <= second[0] && first[1] >= second[1]) || (first[0] >= second[0] && first[1] <= second[1]) 
    };

    let overlap = | first: &Vec<usize>, second: &Vec<usize> | -> bool {
        (first[1] <= second[1] && first[1] >= second[0]) | (second[0] <= first[1] && second[1] >= first[0])
    };
        
    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = ranges
        .iter()
        .filter(|range| {
            contained(&range[0], &range[1])
        })
        .count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = ranges
        .iter()
        .filter(|range| {
            overlap(&range[0], &range[1])
        })
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
