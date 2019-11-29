use std::time::{Instant};
extern crate queues;

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 7 of 2016");

    // let input_file = "./input/day07_16_test.txt";
    // let input_file = "./input/day07_16_test2.txt";
    let input_file = "./input/day07_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut counter = 0;
    for address in &input {
        let mut q: Vec<char> = vec![];
        let mut mode: bool = true;
        let mut valid: bool = false;
        for c in address.chars() {
            if c == '[' {
                mode = false;
            } else if c == ']' {
                mode = true;
            }
            q.push(c);
            if q.len() == 5 {
                q.remove(0);
            }
            if q.len() == 4 {
                if q[0]==q[3] && q[1]==q[2] && q[1]!=q[0] {
                    if mode {
                        valid = true;
                    } else {
                        valid = false;
                        break;
                    }

                }
            }
        }
        if valid {
            counter+=1;
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", counter, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut hypernet: Vec<Vec<char>> = vec![];
    let mut supernet: Vec<Vec<char>> = vec![];

    counter = 0;
    for address in &input {
        let mut q: Vec<char> = vec![];
        let mut mode: bool = true;
        let mut valid: bool = false;
        for c in address.chars() {
            if c == '[' {
                mode = false;
            } else if c == ']' {
                mode = true;
            }
            q.push(c);
            if q.len() == 4 {
                q.remove(0);
            }
            if q.len() == 3 {
                if q[0]==q[2] && q[1]!=q[2] && !(q.contains(&'[') || q.contains(&']')) {
                    if mode {
                        let inverseq: Vec<char> = vec![q[1],q[0],q[1]];
                        hypernet.push(inverseq.to_vec());
                    } else {
                        supernet.push(q.to_vec());
                    }
                }
            }
        }

        for h in &hypernet {
            if supernet.contains(&h) {
                valid = true;
            }
        }
        for s in &supernet {
            if hypernet.contains(&s) {
                valid = true;
            }
        }

        supernet.clear();
        hypernet.clear();

        if valid {
            counter+=1;
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", counter, after2.duration_since(start2));
}
