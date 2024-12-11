use super::tools;
use std::time::Instant;
use std::collections::HashMap;

fn count(num:u64, t: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    let res;
    if cache.contains_key(&(num, t)) {
        return cache[&(num, t)];
    }

    if t == 0 {
        res = 1;
    } else if num == 0 {
        res = count(1, t-1, cache);
    } else if num.to_string().len()%2 == 0 {
        let sn = num.to_string();
        let half = sn.len()/2;
        res = count(sn[..half].parse::<u64>().unwrap(), t-1, cache) +count(sn[half..].parse::<u64>().unwrap(), t-1, cache);
    } else {
        res = count(num*2024, t-1, cache);
    }
    cache.insert((num, t), res);
    res
}

fn count_all(nums: &Vec<u64>, t: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    let mut res = 0;
    for num in nums {
        res += count(*num, t, cache);
    }
    res
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day11-test.txt"
    } else {
        "./input/day11-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let numbers = input[0].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = count_all(&numbers, 25, &mut cache);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = count_all(&numbers, 75, &mut cache);

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
