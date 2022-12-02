use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day02_22_test.txt"
    } else {
        "./input/day02_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let rounds: Vec<(usize, usize)> = input
        .iter()
        .map(|line| {
            let plays = line.split_whitespace().map(|i| i.chars().nth(0).unwrap() as usize).collect::<Vec<usize>>();
            (plays[0]-65, plays[1]-88)
        }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let diff = | p1, p2 | -> usize {
        (p1 as i8 - p2 as i8).rem_euclid(3) as usize
    };

    let score = | p1, p2 | -> usize {
        match diff(p1,p2) {
            2 => 6,
            0 => 3,
            _ => 0,
        }
    }; 

    let res1 = rounds.iter().fold(0, |s, (p1,p2)| {
        s + 1 + p2 + score(*p1,*p2)
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let find_play = | p1, p2 | -> usize {
        match p2 {
            0 => (p1 + 1) % 3,
            1 => p1,
            2 => (p1 as i8 - 1).rem_euclid(3) as usize,
            _ => panic!("oops")
        }
    };

    let res2 = rounds
        .iter()
        .fold(0, |s, &(p1,p2)| {
            let p = find_play(p1,p2);
            s + 1 + p + score(p1,p)
        });

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
