use super::tools;
use std::time::Instant;
use itertools::sorted;

fn count(target: usize, sizes: &mut Vec<usize>, containers: &Vec<usize>, containers_in_target: usize) -> usize {

    if containers.len() == 0 {
        0
    } else {
        let first = containers[0];
        let remain = containers[1..].to_vec();
        let mut with_first = 0;
        if first == target {
            sizes.push(containers_in_target+1);
            with_first = 1;
        } else if first < target {
            with_first = count(target - first, sizes, &remain, containers_in_target+1);
        }
        with_first + count(target, sizes, &remain, containers_in_target)
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17_15_test.txt"
    } else {
        "./input/day17_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let containers: Vec<usize> = sorted(input.iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut sizes = vec![];
    let res1 = count(if !real { 25 } else { 150 }, &mut sizes, &containers, 0);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    sizes = sorted(sizes).collect::<Vec<usize>>();
    let min_size = sizes[0];
    let res2 = sizes.iter().filter(|&s| s==&min_size).count();

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
