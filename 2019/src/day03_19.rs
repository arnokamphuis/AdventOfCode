use std::time::{Instant};
use super::tools;

// use std::fs::File;
// use std::io::{BufWriter, Write};
// use std::io::prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Line {
    b:(i64,i64),
    e:(i64,i64)
}

impl Line {
    fn is_horizontal(&self) -> bool { self.b.1==self.e.1 }
    fn is_vertical(&self) -> bool { self.b.0==self.e.0 }
}

fn get_intersect(l1: &Line, l2: &Line) -> Option<(i64,i64)> {

    if l1.is_horizontal() && l2.is_vertical() {
        let h = l1;
        let v = l2;

        let minh = std::cmp::min(h.b.0, h.e.0); 
        let maxh = std::cmp::max(h.b.0, h.e.0); 
        let minv = std::cmp::min(v.b.1, v.e.1); 
        let maxv = std::cmp::max(v.b.1, v.e.1); 

        if minv < h.b.1 && h.b.1 < maxv && minh < v.b.0 && v.b.0 < maxh {
            Some((v.b.0, h.b.1)) 
        } else {
            None
        }
    } else if l1.is_vertical() && l2.is_horizontal() {
        let h = l2;
        let v = l1;

        let minh = std::cmp::min(h.b.0, h.e.0);
        let maxh = std::cmp::max(h.b.0, h.e.0);
        let minv = std::cmp::min(v.b.1, v.e.1);
        let maxv = std::cmp::max(v.b.1, v.e.1);

        if minv < h.b.1 && h.b.1 < maxv && minh < v.b.0 && v.b.0 < maxh {
            Some((v.b.0, h.b.1)) 
        } else {
            None
        }
    } else {
        None
    }
}

fn get_lines(line: &String) -> Vec<Line> {
    let mut res : Vec<Line> = vec![];
    let mut c_pos: (i64,i64) = (0,0);

    for linemove in line.split(",").map(|s| s.to_owned()).collect::<Vec<String>>().into_iter() {
        if let Some(direction) = linemove.chars().nth(0) {
            if let Ok(distance) = (&linemove[1..]).parse::<i64>() {
                let mut line = Line{b: c_pos, e: c_pos};
                match direction {
                    'L' => { line.e.0 -= distance; }
                    'R' => { line.e.0 += distance; }
                    'U' => { line.e.1 += distance; }
                    'D' => { line.e.1 -= distance; }
                    _ => {}
                }
                res.push(line.clone());
                c_pos = line.e;
            }
        }
    }
    res
}

pub fn run() {
    println!("Day 03 of 2019");

    // let input_file = "./input/day03_19_test.txt";
    let input_file = "./input/day03_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let lines1 = get_lines(&input[0]);
    let lines2 = get_lines(&input[1]);

    // let mut file = File::create("line1.txt").unwrap();
    // let mut writer = BufWriter::new(file);
    // for i in 0..lines1.len() {
    //     let l = lines1[i];
    //     writer.write_all(format!("{} {}\n{} {}\n\n", l.b.0, l.b.1, l.e.0, l.e.1).as_ref() );
    // }

    // let mut file = File::create("line2.txt").unwrap();
    // let mut writer = BufWriter::new(file);
    // for i in 0..lines2.len() {
    //     let l = lines2[i];
    //     writer.write_all(format!("{} {}\n{} {}\n\n", l.b.0, l.b.1, l.e.0, l.e.1).as_ref() );
    // }

    let start1 = Instant::now();

    let mut mindist = std::i64::MAX;
    for i in 0..lines1.len() {
        let l1 = lines1[i];
        for j in 0..lines2.len() {
            let l2 = lines2[j];
            if let Some(inter) = get_intersect(&l1,&l2) {
                if !(inter.0 == 0 && inter.1 == 0) {
                    let dist = inter.0.abs() + inter.1.abs();
                    if mindist > dist {
                        mindist = dist;
                    }
                }
            }
        }
    }

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", mindist, after1.duration_since(start1));

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", 0, after2.duration_since(start2));
}
