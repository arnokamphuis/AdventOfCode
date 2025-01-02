use super::tools;
use std::time::Instant;

use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day04-test.txt"
    } else {
        "./input/day04-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let size = (input[0].len() as i32, input.len() as i32);

    let mut card: HashMap<(i32, i32), char> = HashMap::new();
    input
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            x.chars().enumerate().for_each(|(j, c)| {
                card.insert((j as i32,i as i32), c);
            });
        });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let dirs: Vec::<(i32,i32)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let target = [Some(&'M'), Some(&'A'), Some(&'S')];

    let mut res1 = 0;
    for r in 0..size.1 {
        for c in 0..size.0 {
            if card.get(&(c,r)) == Some(&'X') {
                for d in dirs.iter() {
                    if (1..target.len()+1).map(|i| card.get(&(c + i as i32 *d.0, r+i as i32 * d.1))).collect::<Vec<Option<&char>>>() == target {
                        res1 += 1;
                    }
                }
            }
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let target = [Some(&'M'), Some(&'M'), Some(&'S'), Some(&'S')];
    let offsets: Vec<(i32,i32)> = vec![(-1,-1), (-1,1), (1, 1), (1, -1)];
    let mut res2 = 0;
    for r in 0..size.1 {
        for c in 0..size.0 {
            if card.get(&(c,r)) == Some(&'A') {
                for i in 0..4 {
                    let working_offset = [&offsets[i..],&offsets[..i]].concat();
                    if working_offset
                        .iter()
                        .map(|(dc, dr)| card.get(&(c + dc, r + dr)))
                        .collect::<Vec<Option<&char>>>() == target {
                            res2 += 1;
                        }
                }
            }
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
