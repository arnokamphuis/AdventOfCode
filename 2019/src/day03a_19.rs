use std::time::{Instant};
use super::tools;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}


fn get_points(line: &String) -> HashMap<Point, i64> {
    let mut res : HashMap<Point, i64> = HashMap::new();
    let mut c_pos = Point{x:0,y:0};
    let mut d = 0;

    for linemove in line.split(",").map(|s| s.to_owned()).collect::<Vec<String>>().into_iter() {
        if let Some(direction) = linemove.chars().nth(0) {
            if let Ok(distance) = (&linemove[1..]).parse::<i64>() {
                for _ in 0..distance {
                    d += 1;
                    match direction {
                        'L' => { c_pos.x -= 1; }
                        'R' => { c_pos.x += 1; }
                        'U' => { c_pos.y += 1; }
                        'D' => { c_pos.y -= 1; }
                        _ => {}
                    }
                    res.insert(c_pos, d);
                }
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

    let points1 = get_points(&input[0]);
    let points2 = get_points(&input[1]);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut mindist: i64 = std::i64::MAX;
    let mut mindura: i64 = std::i64::MAX;

    points1.iter().for_each( |(p1, _)| if points2.contains_key(p1) {
        mindist = std::cmp::min(p1.x.abs() + p1.y.abs(), mindist);
    });

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", mindist, after1.duration_since(start1));

    let start2 = Instant::now();

    points1.iter().for_each( |(p1,d)| if points2.contains_key(p1) {        
        mindura = std::cmp::min(d + points2[p1], mindura); 
    });

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", mindura, after2.duration_since(start2));
}
