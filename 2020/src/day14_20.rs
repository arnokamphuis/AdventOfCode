use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;
use regex::Regex;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_20_test.txt"
    } else {
        "./input/day14_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut mem: BTreeMap<usize, u64> = BTreeMap::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mask_regex = Regex::new(r"mask = ([X10]+)").expect("Invalid regex");
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Invalid regex");

    for line in &input {
        if mask_regex.is_match(line) {
            let captures = mask_regex.captures_iter(line);
            for cap in captures {
                let mask = cap.get(1).unwrap().as_str();
                or_mask = 0;
                and_mask = 0;
                for c in mask.chars() {
                    match c {
                        'X' => { or_mask = (or_mask << 1) + 0; and_mask = (and_mask << 1) + 1; },
                        '1' => { or_mask = (or_mask << 1) + 1; and_mask = (and_mask << 1) + 1; },
                        '0' => { or_mask = (or_mask << 1) + 0; and_mask = (and_mask << 1) + 0; },
                        _ => {}
                    }
                }
            }
        }
        if mem_regex.is_match(line) {
            let captures = mem_regex.captures_iter(line);
            for cap in captures {
                let address = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

                mem.entry(address).or_insert(0);
                if let Some(m) = mem.get_mut(&address) {
                    *m = (value & and_mask) | or_mask;
                }
            }
        }
    }

    let res1 = mem.iter().fold(0, |acc, v| acc + *v.1 );

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    mem.clear();
    let mut x_pos = vec![];
    for line in &input {
        if mask_regex.is_match(line) {
            x_pos = vec![];
            let captures = mask_regex.captures_iter(line);
            for cap in captures {
                let mask = cap.get(1).unwrap().as_str();
                or_mask = 0;
                for (i,c) in mask.chars().enumerate() {
                    match c {
                        'X' => { or_mask = (or_mask << 1) + 0; x_pos.push(35-i) },
                        '1' => { or_mask = (or_mask << 1) + 1 },
                        '0' => { or_mask = (or_mask << 1) + 0 },
                        _ => {}
                    }
                }
            }
        }
        if mem_regex.is_match(line) {
            let captures = mem_regex.captures_iter(line);
            for cap in captures {
                let address = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

                for t in 0..(1 << x_pos.len()) {
                    let mut new_addr = address | or_mask;
                    let mut tb: usize = 0;
                    for b in &x_pos {
                        if t & (1 << tb) > 0 {
                            new_addr |= 1 << b;
                        } else {
                            new_addr &= !(1 << b);
                        }
                        tb += 1;
                    }
                    mem.entry(new_addr as usize).or_insert(0);
                    if let Some(m) = mem.get_mut(&(new_addr as usize)) { *m = value; }
                }
            }
        }
    }

    let res2 = mem.iter().fold(0, |acc, v| acc + *v.1 );

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
