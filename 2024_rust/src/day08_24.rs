use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn in_bounds(a: (i32,i32), max_r: i32, max_c: i32) -> bool {
    return a.0 >= 0 && a.0 < max_c && a.1 >= 0 && a.1 < max_r;
}

fn find_antinodes_harmonics(a: &(i32,i32), b: &(i32,i32), max_r: i32, max_c: i32, harmonics: bool) -> HashSet<(i32,i32)> {
    let mut antinodes: HashSet<(i32,i32)> = HashSet::from_iter( (if harmonics { vec![*a] } else { vec![] }).into_iter() );

    let d = (a.0 - b.0, a.1 - b.1);
    let mut antinode = (a.0 + d.0, a.1 + d.1);

    let mut step = 1;
    while in_bounds(antinode, max_r, max_c) {
        antinodes.insert(antinode);
        if harmonics {
            step += 1;
            antinode = (a.0 + step * d.0, a.1 + step * d.1);
        } else {
            break;
        }
    }

    return antinodes;
}

fn count_antinodes(map: &HashMap<u8, Vec<(i32,i32)>>, r: i32, c: i32, harmonics: bool) -> usize {
    return map
        .iter()
        .fold(HashSet::<(i32,i32)>::new(), | acc, (_, antennas)| {
            antennas.iter().combinations(2).fold(acc, |cur_set, v| {
                cur_set
                    .union(&find_antinodes_harmonics(v[0], v[1], r, c, harmonics))
                    .cloned()
                    .collect::<HashSet<(i32,i32)>>()
                    .union(&find_antinodes_harmonics(v[1], v[0], r, c, harmonics))
                    .cloned()
                    .collect::<HashSet<(i32,i32)>>()
            })
        }).len();
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

    let res1 = count_antinodes(&map, r, c, false);

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = count_antinodes(&map, r, c, true);

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
