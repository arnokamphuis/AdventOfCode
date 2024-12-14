use super::tools;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;

fn calculate_position(time: i32, (c,r): (i32, i32), ((x,y),(vx,vy)): ((i32,i32),(i32,i32))) -> (i32,i32) {
    ((x + time*vx).rem_euclid(c), (y + time*vy).rem_euclid(r))
}

fn get_quadrants(pos:(i32,i32), size: (i32, i32)) -> usize {
    let mid = (size.0/2, size.1/2);
    match pos {
        (x,y) if x < mid.0 && y < mid.1 => 1,
        (x,y) if x > mid.0 && y < mid.1 => 2,
        (x,y) if x < mid.0 && y > mid.1 => 3,
        (x,y) if x > mid.0 && y > mid.1 => 4,
        _ => 0
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14-test.txt"
    } else {
        "./input/day14-real.txt"
    };

    let input = tools::get_input(String::from(input_file));

    let re = Regex::new(r"[-+]?\d+").unwrap();
    let robots: Vec<((i32,i32),(i32,i32))> = input
        .iter()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .into_iter()
                .collect_tuple()
                .unwrap()
        })
        .map(|(x,y,vx,vy)| ((x,y),(vx,vy)))
        .collect_vec();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let (r,c) = if !real { (11, 7) } else { (101, 103) };

    let res1 = robots
        .iter()
        .map(|p| calculate_position(100, (r,c), *p))
        .map(|p| get_quadrants(p, (r,c)))
        .fold(vec![0;5], |mut acc, x| { acc[x] += 1; acc })
        .iter().skip(1)
        .product::<usize>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = (0..(r*c)).fold((0, usize::MAX), |min, time| {
        let moment = (time, robots
            .iter()
            .map(|p| calculate_position(time, (r,c), *p))
            .map(|p| get_quadrants(p, (r,c)))
            .fold(vec![0;5], |mut acc, x| { acc[x] += 1; acc })
            .iter().skip(1)
            .product::<usize>());
        if moment.1 < min.1 {
            moment
        } else {
            min
        }
    }).0;

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
