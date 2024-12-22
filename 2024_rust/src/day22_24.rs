use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use itertools::Itertools;

fn apply(secret: u64) -> u64 {
    let mut s = secret;
    s = ((s << 6) ^ s) % 16777216;
    s = ((s >> 5) ^ s) % 16777216;
    s = ((s << 11) ^ s) % 16777216;
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

    let real_prices = prices
        .iter()
        .map(|x| x.iter().map(|s| *s % 10).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();

    let changes: Vec<Vec<i64>> = real_prices
        .iter()
        .map(|v| {
            v
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, s)| { *s as i64 - v[i] as i64 })
                .collect::<Vec<i64>>()
    
        })
        .collect::<Vec<Vec<i64>>>();
    
    let values = changes
        .iter()
        .enumerate()
        .map(|(vi, v)| {
            v
                .iter()
                .tuple_windows::<(_,_,_,_)>()
                .enumerate()
                .fold(HashMap::new(), |mut vals, (i, (&p1,&p2,&p3,&p4))| {
                    if vals.contains_key(&(p1,p2,p3,p4)) {
                        vals
                    } else {
                        vals.insert((p1,p2,p3,p4), real_prices[vi][i+4]);
                        vals
                    }
                })
        })
        .collect::<Vec<HashMap<(i64,i64,i64,i64), u64>>>();

    let res2 = values
        .iter()
        .fold(HashMap::new(), |mut map, v| {
            v.iter().for_each(|(k, v)| {
                *map.entry(*k).or_insert(0) += v;
            });
            map
        })
        .iter()
        .map(|(_,v)| { *v })
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
