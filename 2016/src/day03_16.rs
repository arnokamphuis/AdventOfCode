// use std::collections::HashSet;
use std::time::{Instant};
use order_stat;
// use std::char;

use super::tools;

#[allow(dead_code)]
pub fn run() {
    println!("Day 3 of 2016");

    // let input_file = "./input/day03_16_test.txt";
    // let input_file = "./input/day03_16_test2.txt";
    let input_file = "./input/day03_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(String::from(&value));
            }
        }
    }

    let mut validcount = 0;
    for line in &input {
        let mut triangle = [0,0,0];
        let mut index = 0;
        for value in line.split_whitespace() {            
            triangle[index] = value.parse::<i64>().unwrap();
            index+=1;
        }
        let smallest = *order_stat::kth(&mut triangle, 0);
        let middle   = *order_stat::kth(&mut triangle, 1);
        let largest  = *order_stat::kth(&mut triangle, 2);

        if (smallest+middle) > largest {
            validcount += 1;
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", validcount, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut validcount = 0;

    let mut triangles = [[0,0,0],[0,0,0],[0,0,0]];

    let mut linecount = 0;
    for line in &input {

        let mut index = 0;
        for value in line.split_whitespace() {            
            triangles[linecount][index] = value.parse::<i64>().unwrap();
            index+=1;
        }
        linecount += 1;

        if linecount == 3 {
            for t in 0..3 {
                let mut triangle = [0,0,0];
                for s in 0..3 {
                    triangle[s] = triangles[s][t];
                }
                let smallest = *order_stat::kth(&mut triangle, 0);
                let middle   = *order_stat::kth(&mut triangle, 1);
                let largest  = *order_stat::kth(&mut triangle, 2);
                
                if (smallest+middle) > largest {
                    validcount += 1;
                }
            }
            linecount=0;
        }
    }


    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", validcount, after2.duration_since(start2));
}
