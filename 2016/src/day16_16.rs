use std::time::Instant;
use std::io;
use std::io::Write;
extern crate itertools;

use itertools::Itertools;

fn extend(bits: &String) -> String {
    let mut res: String = bits.clone();
    res.push('0');
    bits.chars().rev().map(|b| if b=='1' { '0'} else { '1'} ).for_each(|c| res.push(c));
    res
}

fn calculate_checksum(start: &String, size: usize) -> String {
    let mut input = start.clone();

    while input.len() < size {
        input = extend(&input);
    }

    input = input[0..size].to_string();
    
    while input.len() % 2 == 0 {
        io::stdout().flush().unwrap();
        let mut checksum = String::from("");
        input.chars().chunks(2).into_iter().for_each(|pair| { let p = pair.collect::<Vec<_>>(); checksum.push( if p[0]==p[1] { '1' } else { '0' } )});
        input = checksum.clone();
    }
    input.clone()
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 16 of 2016");

    let start0 = Instant::now();

    let input = String::from("10001001100000001");

    let after0 = Instant::now();
    println!(
        "Init in {:?}",
        after0.duration_since(start0)
    );

    let start1 = Instant::now();

    let mut size = 272;
    let res1 = calculate_checksum(&input, size);

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        res1,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    size = 35651584;
    let res2 = calculate_checksum(&input, size);

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        res2,
        after2.duration_since(start2)
    );
}
