use std::time::{Instant};

use super::tools;

fn calculate_finalsize(code: &mut String) -> (u64, usize) {
  let mut finalsize: u64 = 0;
  let mut offset = 0;

      while offset < code.len() {
        match code[offset..].find('(') {
          None => { 
            let shift = code[offset..].len(); 
            finalsize += shift as u64; 
            offset += shift; 
          }
          Some(v) => {
            let posstart = v;
            let posstop  = code[offset..].find(')').unwrap();
            let sizestr = code[offset..].get(posstart+1..posstop).unwrap();
            let mut size = sizestr.split('x');
            let numchars = size.next().unwrap().parse::<usize>().unwrap();
            let repeat = size.next().unwrap().parse::<u64>().unwrap();

            let mut subcode = String::from(code[offset..].get(posstop+1..posstop+1+numchars).unwrap());
            let subsize = calculate_finalsize(&mut subcode);

            finalsize += posstart as u64;
            finalsize += (subsize.0 * repeat) as u64;
            offset += numchars + posstop+1;
          }
        }
      }

  (finalsize, offset)
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 9 of 2016");

    // let input_file = "./input/day09_16_test.txt";
    // let input_file = "./input/day09_16_test2.txt";
    let input_file = "./input/day09_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut finalsize = 0;
    let mut offset = 0;
    for code in &input {
      finalsize = 0;
      while offset < code.len() {
        match code[offset..].find('(') {
          None => { 
            let shift = code[offset..].len(); 
            finalsize += shift; 
            offset += shift; 
          }
          Some(v) => {
            let posstart = v;
            let posstop  = code[offset..].find(')').unwrap();
            let sizestr = code[offset..].get(posstart+1..posstop).unwrap();
            let mut size = sizestr.split('x');
            let numchars = size.next().unwrap().parse::<usize>().unwrap();
            let repeat = size.next().unwrap().parse::<usize>().unwrap();

            finalsize += posstart;
            finalsize += numchars * repeat;
            offset += numchars + posstop+1;
          }
        }
      }
      offset = 0;
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", finalsize, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut finalsize2:u64 = 0;
    for code in &input {
      let mut realcode = String::from(code);
      finalsize2 = calculate_finalsize(&mut realcode).0;
    }
    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", finalsize2, after2.duration_since(start2));
}
