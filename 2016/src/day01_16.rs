use std::collections::HashSet;
use std::time::{Instant};

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 1 of 2016");

    // let input_file = "./input/day01_16_test.txt";
    let input_file = "./input/day01_16_real.txt";

    let start1 = Instant::now();

    let mut current_dir = 'n';
    let mut prev_pos = (0,0);
    let mut current_pos = (0,0);
    let mut positions = HashSet::new();
    let mut revisited = (0,0);
    let mut foundone: bool = false;

    positions.insert(current_pos);

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {

                for v in value.split(", ") {
                    let direction = v.chars().next().unwrap();
                    let delta = v[1..].parse::<i64>().unwrap();
                    match direction {
                        'R' => {
                            match current_dir {
                                'n' => { current_dir = 'e'; current_pos.0 += delta; }
                                'e' => { current_dir = 's'; current_pos.1 -= delta; }
                                's' => { current_dir = 'w'; current_pos.0 -= delta; }
                                'w' => { current_dir = 'n'; current_pos.1 += delta; }
                                _ => { println!("ERROR"); }
                            }
                        }
                        'L' => {
                            match current_dir {
                                'n' => { current_dir = 'w'; current_pos.0 -= delta; }
                                'e' => { current_dir = 'n'; current_pos.1 += delta; }
                                's' => { current_dir = 'e'; current_pos.0 += delta; }
                                'w' => { current_dir = 's'; current_pos.1 -= delta; }
                                _ => { println!("ERROR"); }
                            }

                        }
                        _ => { println!("ERROR IN DIRECTION");}
                    }

                    let mut dvec = (0,0);
                    dvec.0 = (current_pos.0 - prev_pos.0)/delta;
                    dvec.1 = (current_pos.1 - prev_pos.1)/delta;
                    for _i in 0..delta {
                        prev_pos.0 += dvec.0;
                        prev_pos.1 += dvec.1;
                        if !foundone && positions.contains(&prev_pos) {
                            revisited = prev_pos;
                            foundone = true;
                        }
                        positions.insert(prev_pos);
                    }
                    prev_pos = current_pos;
                }
            }
        }
    }


    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", (current_pos.0.abs() + current_pos.1.abs()), after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}",  (revisited.0.abs() + revisited.1.abs()), after2.duration_since(start2));
}
