use std::time::{Instant};

use super::tools;

pub fn run() {
    println!("Day 1 of 2016");

    // let input_file = "./input/day01_16_test.txt";
    let input_file = "./input/day01_16_real.txt";

    let start1 = Instant::now();

    let mut current_dir = 'n';
    let mut current_pos = (0,0);

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {

                for v in value.split(", ") {
                    let direction = v.chars().next().unwrap();
                    match direction {
                        'R' => {
                            match current_dir {
                                'n' => { current_dir = 'e'; }
                                'e' => { current_dir = 's'; }
                                's' => { current_dir = 'w'; }
                                'w' => { current_dir = 'n'; }
                                _ => { println!("ERROR"); }
                            }
                        }
                        'L' => {
                            match current_dir {
                                'n' => { current_dir = 'w'; }
                                'e' => { current_dir = 'n'; }
                                's' => { current_dir = 'e'; }
                                'w' => { current_dir = 's'; }
                                _ => { println!("ERROR"); }
                            }

                        }
                        _ => { println!("ERROR IN DIRECTION");}
                    }
                    let delta = v[1..].parse::<i64>().unwrap();

                    match current_dir {
                        'n' => { current_pos.1 += delta; }
                        'e' => { current_pos.0 += delta; }
                        's' => { current_pos.1 -= delta; }
                        'w' => { current_pos.0 -= delta; }
                        _ => { println!("ERROR"); }
                    }
                }
            }
        }
    }


    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", (current_pos.0.abs() + current_pos.1.abs()), after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}
