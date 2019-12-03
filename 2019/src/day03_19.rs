use std::time::{Instant};
use super::tools;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Line {
    b:(i64,i64),
    e:(i64,i64),
    bt: i64,
    et: i64,
}

impl Line {
    fn is_horizontal(&self) -> bool { self.b.1==self.e.1 }
    fn is_vertical(&self) -> bool { self.b.0==self.e.0 }
}

fn get_intersect(l1: &Line, l2: &Line) -> Option<((i64,i64), (i64,i64))> {

    if l1.is_horizontal() && l2.is_vertical() {
        let h = l1;
        let v = l2;

        let minh = std::cmp::min(h.b.0, h.e.0); 
        let maxh = std::cmp::max(h.b.0, h.e.0); 
        let minv = std::cmp::min(v.b.1, v.e.1); 
        let maxv = std::cmp::max(v.b.1, v.e.1); 

        if minv < h.b.1 && h.b.1 < maxv && minh < v.b.0 && v.b.0 < maxh {
            let d1 = (v.b.0 - h.b.0).abs() + h.bt;
            let d2 = (h.b.1 - v.b.1).abs() + v.bt;
            if v.b.0 == 0 && h.b.1 == 0 {
                None
            } else {
                Some(((v.b.0, h.b.1), (d1,d2))) 
            }
        } else {
            None
        }
    } else if l1.is_vertical() && l2.is_horizontal() {
        let v = l1;
        let h = l2;

        let minh = std::cmp::min(h.b.0, h.e.0);
        let maxh = std::cmp::max(h.b.0, h.e.0);
        let minv = std::cmp::min(v.b.1, v.e.1);
        let maxv = std::cmp::max(v.b.1, v.e.1);

        if minv < h.b.1 && h.b.1 < maxv && minh < v.b.0 && v.b.0 < maxh {
            let d1 = (v.b.1 - h.b.1).abs() + v.bt;
            let d2 = (h.b.0 - v.b.0).abs() + h.bt;
            if v.b.0 == 0 && h.b.1 == 0 {
                None
            } else {
                Some(((v.b.0, h.b.1), (d1,d2))) 
            }
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
    let mut c_dist: i64 = 0;

    for linemove in line.split(",").map(|s| s.to_owned()).collect::<Vec<String>>().into_iter() {
        if let Some(direction) = linemove.chars().nth(0) {
            if let Ok(distance) = (&linemove[1..]).parse::<i64>() {
                let mut line = Line{b: c_pos, e: c_pos, bt: c_dist, et: c_dist + distance };
                match direction {
                    'L' => { line.e.0 -= distance; }
                    'R' => { line.e.0 += distance; }
                    'U' => { line.e.1 += distance; }
                    'D' => { line.e.1 -= distance; }
                    _ => {}
                }

                res.push(line.clone());
                c_pos = line.e;
                c_dist += distance;
            }
        }
    }
    res
}

pub fn run() {
    println!("Day 03 of 2019");

    let start0 = Instant::now();
    // let input_file = "./input/day03_19_test.txt";
    let input_file = "./input/day03_19_real.txt";

    let input = tools::get_input(String::from(input_file));

    let lines1 = get_lines(&input[0]);
    let lines2 = get_lines(&input[1]);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut mindist = std::i64::MAX;
    let mut intersections: Vec<((i64,i64), (i64, i64))> = vec![];

    lines1.iter().for_each( |l1|
        lines2.iter().for_each( |l2|
            if let Some(inter) = get_intersect(&l1,&l2) {
                mindist = std::cmp::min((inter.0).0.abs() + (inter.0).1.abs(),mindist);
                intersections.push( inter ); // store for part 2
            }
        )
    );

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", mindist, after1.duration_since(start1));

    let mut min_duration: i64 = std::i64::MAX;
    intersections.iter().for_each( |inter| min_duration = std::cmp::min(min_duration, (inter.1).0 + (inter.1).1) );

    let start2 = Instant::now();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", min_duration, after2.duration_since(start2));
}
