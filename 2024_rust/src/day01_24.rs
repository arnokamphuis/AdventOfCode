use super::tools;
use std::time::Instant;
// use itertools::sorted;
use itertools::izip;
use counter::Counter;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day01-test.txt"
    } else {
        "./input/day01-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let [ref ll, ref rl] = input
        .iter()
        .fold(vec![vec![], vec![]], |mut v, line| {
            line.split_whitespace().enumerate().for_each(|(i, value)| {
                v[i].push(value.parse::<i32>().unwrap());
            });
            v.iter_mut().for_each(|l| l.sort());
            v
        })[0..2] else {
            panic!("Invalid input");
        };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = izip!(ll, rl)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let right_count = rl.iter().collect::<Counter<&i32>>();
    let res2 = ll.iter().map(|v| v * right_count[v] as i32).sum::<i32>();

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
