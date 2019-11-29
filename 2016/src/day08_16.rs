use std::time::{Instant};

use super::tools;

fn count_lights(display: &Vec<Vec<bool>>) -> u64 {
  let mut count:u64 = 0;
  for r in display {
    for c in r {
      if *c {
        count+=1;
      }
    }
  }
  count
}

fn perform_rect(display : &mut Vec<Vec<bool>>, w: usize, h: usize) {
  for x in 0..w {
    for y in 0..h {
      display[x][y] = true;
    }
  }
}

fn perform_rotate_row(display : &mut Vec<Vec<bool>>, y: usize, delta: usize) {
  let size = (display.len(), display[0].len());
  let mut row = vec![false; size.0];
  for x in 0..size.0 {
    row[(x+delta)%size.0] = display[x][y];
  }
  for x in 0..size.0 {
    display[x][y] = row[x];
  }
}

fn perform_rotate_column(display : &mut Vec<Vec<bool>>, x: usize, delta: usize) {
  let size = (display.len(), display[0].len());
  let mut col = vec![false; size.1];
  for y in 0..size.1 {
    col[(y+delta)%size.1] = display[x][y];
  }
  for y in 0..size.1 {
    display[x][y] = col[y];
  }
}


fn print_display(display : &Vec<Vec<bool>>) {
  let size = (display.len(), display[0].len());
  for y in 0..size.1 {
    let mut row = String::from("");
    for x in 0..size.0 {
      let mut c = '.';
      if display[x][y] {
        c = '#';
      }
      row.push(c);
    }
    println!("{}",row);
  }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 8 of 2016");

    // let input_file = "./input/day08_16_test.txt";
    let input_file = "./input/day08_16_real.txt";

    let start1 = Instant::now();

    let mut input = Vec::new();

    if let Ok(lines) = tools::read_lines(input_file) {
        for line in lines {
            if let Ok(value) = line {
                input.push(value);
            }
        }
    }

    let mut display: Vec<Vec<bool>> = vec![];
    let size:(usize,usize) = (50,6);

    for _ in 0..size.0 {
      display.push(vec![false; size.1]);
    }


    for operation in &input {
      let mut tokens = operation.split_whitespace();
      let op = tokens.next().unwrap();
      match op {
        "rect" => {
          let rectsize = tokens.next().unwrap();
          let mut sizetokens = rectsize.split("x");
          let w = sizetokens.next().unwrap().parse::<usize>().unwrap();
          let h = sizetokens.next().unwrap().parse::<usize>().unwrap();
          perform_rect(&mut display, w, h);
        }
        "rotate" => {
          let direction = tokens.next().unwrap();
          let pos = tokens.next().unwrap()[2..].parse::<usize>().unwrap();
          tokens.next();
          let d = tokens.next().unwrap().parse::<usize>().unwrap();
          match direction {
            "column" => {
              perform_rotate_column(&mut display, pos, d);
            }
            "row" => {
              perform_rotate_row(&mut display, pos, d);              
            }
            _ => {}
          }
        }
        _ => {}
      }
    }

    let count = count_lights(&display);

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", count, after1.duration_since(start1));

    let start2 = Instant::now();

    print_display(&display);

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}
