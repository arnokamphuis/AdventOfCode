use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn apply(secret: u64) -> u64 {
    let mut s = secret;
    s = ((s <<  6) ^ s) & 0xFFFFFF;
    s = ((s >>  5) ^ s) & 0xFFFFFF;
    s = ((s << 11) ^ s) & 0xFFFFFF;
    return s
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day22-test.txt"
    } else {
        "./input/day22-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let secrets = input.iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let prices: Vec<Vec<u64>> = secrets
        .iter()
        .map(|s| {
            (0..2000).fold(vec![*s], |mut acc, _| {
                acc.push(apply(*acc.last().unwrap()));
                acc
            })
        })
        .collect::<Vec<Vec<u64>>>();

    let last_prices = prices
        .iter()
        .map(|x| *x.last().unwrap())
        .collect::<Vec<u64>>();

    let res1: u64 = last_prices.iter().sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = prices
        .iter()
        .map(|x| x.iter().map(|s| *s % 10).collect::<Vec<u64>>())
        .map(|v| {
            v
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, s)| { *s as i64 - v[i] as i64 })
                .collect::<Vec<i64>>()
    
        })
        .enumerate()
        .fold((HashMap::new(), vec![HashSet::<(_,_,_,_)>::new() ; prices.len()]), |(mut map, mut done), (vi, v)| {
            v
                .iter()
                .tuple_windows::<(_,_,_,_)>()
                .enumerate()
                .for_each(|(i, (&p1,&p2,&p3,&p4))| {
                    if !done[vi].contains(&(p1,p2,p3,p4)) {
                        done[vi].insert((p1,p2,p3,p4));
                        *map.entry((p1,p2,p3,p4)).or_insert(0) += prices[vi][i+4] % 10;
                    }
                });
            (map, done)
        })
        .0
        .iter()
        .map(|(_,v)| *v )
        .max().unwrap();

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
