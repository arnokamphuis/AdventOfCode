use super::tools;
use std::time::Instant;
use std::collections::{HashSet};
// use itertools::Itertools;

fn solve(res: i64, nums:Vec<i64>, concat: bool) -> bool {
    if nums.len() == 1 {
        return nums[0] == res;
    }

    let last = nums[nums.len()-1];

    if res % last == 0 {
        if solve(res / last, nums[0..nums.len()-1].to_vec(), concat) {
            return true;
        }
    }

    let size = last.ilog10()+1;
    if res > last {
        if solve(res - last, nums[0..nums.len()-1].to_vec(), concat) {
            return true;
        }
    }

    if concat && res > 10_i64.pow(size) {
        let remaining = res - (res / 10_i64.pow(size)) * 10_i64.pow(size);
        if remaining == last {
            if solve(res / 10_i64.pow(size), nums[0..nums.len()-1].to_vec(), concat) {
                return true;
            }
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07-test.txt"
    } else {
        "./input/day07-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut equations: HashSet<(Vec<i64>, i64)> = HashSet::new();
    input
        .iter()
        .for_each(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let result = parts[0].parse::<i64>().unwrap();
            let eqstr = parts[1].split_whitespace().collect::<Vec<&str>>();
            let eq = eqstr.iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            equations.insert((eq, result));
        });
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    for (eq, res) in equations.iter() {
        if solve(*res, eq.clone(), false) {
            res1 += res;
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    for (eq, res) in equations.iter() {
        if solve(*res, eq.clone(), true) {
            res2 += res;
        }
    }

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
