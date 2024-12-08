use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn in_bounds(a: (i32,i32), max_r: i32, max_c: i32) -> bool {
    return a.0 >= 0 && a.0 < max_c && a.1 >= 0 && a.1 < max_r;
}

fn find_antinodes_harmonics(a: &(i32,i32), b: &(i32,i32), max_r: i32, max_c: i32, harmonics: bool) -> Vec<(i32,i32)> {
    let mut antinodes: Vec<(i32,i32)> = vec![];

    let dr = a.0 - b.0;
    let dc = a.1 - b.1;

    let mut l = 1;
    let mut antinode = (a.0 + l*dr, a.1 + l*dc);
    while in_bounds(antinode, max_r, max_c) {
        if !harmonics && antinode == *b {
            break;
        }

        antinodes.push(antinode);

        if !harmonics {
            break;
        }
        l += 1;
        antinode = (a.0 + l*dr, a.1 + l*dc);
    }

    if harmonics {
        antinodes.push(*a);
    }

    return antinodes;
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08-test.txt"
    } else {
        "./input/day08-real.txt"
    };

    let input = tools::get_input(String::from(input_file));
    let (r, c) = (input.len() as i32, input[0].len() as i32);

    let mut map: HashMap<u8, Vec<(i32,i32)>> = HashMap::new();
    input.iter().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            if ch != '.' {
                map.entry(ch as u8).or_insert(vec![]).push((c as i32, r as i32));
            }
        });
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut all_antinodes: HashSet<(i32,i32)> = HashSet::new();
    for freq in map.keys() {
        let antennas: &Vec<(i32,i32)> = map.get(freq).unwrap();
        for a in antennas.iter() {
            for b in antennas.iter() {
                if a != b {
                    all_antinodes.extend(
                        find_antinodes_harmonics(a, b, r, c, false));
                    all_antinodes.extend(
                        find_antinodes_harmonics(b, a, r, c, false));
                }
            }
        }
    }
    let res1 = all_antinodes.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut all_antinodes: HashSet<(i32,i32)> = HashSet::new();
    for freq in map.keys() {
        let antennas: &Vec<(i32,i32)> = map.get(freq).unwrap();
        for a in antennas.iter() {
            for b in antennas.iter() {
                if a != b {
                    all_antinodes.extend(
                        find_antinodes_harmonics(a, b, r, c, true));
                    all_antinodes.extend(
                        find_antinodes_harmonics(b, a, r, c, true));
                }
            }
        }
    }
    let res2 = all_antinodes.len();

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
