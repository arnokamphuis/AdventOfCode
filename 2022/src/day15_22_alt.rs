use super::tools;
use std::time::Instant;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15_22_test.txt"
    } else {
        "./input/day15_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let sensors = input.iter().map(|line| {
        let words = line.split(": closest beacon is at ").collect::<Vec<&str>>();
        (words[0][10..].split(", ").map(|s| s[2..].parse::<i64>().unwrap()).collect::<Vec<i64>>(),
        words[1].split(", ").map(|s| s[2..].parse::<i64>().unwrap()).collect::<Vec<i64>>() )
    }).collect::<Vec<_>>();

    let sensor_distance = sensors
        .iter()
        .map(|(s,b)| { (s, (s[0]-b[0]).abs() + (s[1]-b[1]).abs()) } )
        .collect::<Vec<_>>();


    let range_union = | sets: &Vec<(i64,i64)> | -> Vec<(i64,i64)> {
        let mut sorted_sets = sets.clone();
        sorted_sets.sort_by(|a,b| a.0.cmp(&b.0));
        let mut res: Vec<(i64,i64)> = vec![sorted_sets[0]];
        sorted_sets.iter().skip(1).for_each(|iv| {
            let l = res.len()-1;
            if res[l].1 < iv.0 {
                res.push(*iv);
            } else if res[l].1 >= iv.0 {
                if res[l].1 < iv.1 {
                    res[l].1 = iv.1;
                }
            }
        });
        res
    };

    let beacon_distance = | y_val: i64 | -> Vec<_> {
        sensor_distance
            .iter()
            .filter(|(s,d)| { d > &(y_val-s[1]).abs() })
            .map(|(s,d)| (s,d-(y_val-s[1]).abs()))
            .map(|(s,d)| (s[0]-d, s[0]+d))
            .collect::<Vec<_>>()
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let scan_line = if !real { 10 } else { 2_000_000 };
    let beacons_on_scanline = sensors
        .iter()
        .filter(|(_,b)| b[1] == scan_line)
        .map(|(_,b)| b)
        .collect::<HashSet<_>>();

    let impossible_range = range_union(&beacon_distance(scan_line));   
    let res1 = impossible_range
        .iter()
        .fold(0, |acc, range| acc + range.1 - range.0 + 1) - beacons_on_scanline.len() as i64;

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let max_range = if !real { 20 } else { 4_000_000 };

    let valid = | x: i64, y: i64, sd: &Vec<(&Vec<i64>,i64)> | -> bool {
        for (s,d) in sd {
            let dxy = (x-s[0]).abs() + (y-s[1]).abs();
            if dxy <= *d {
                return false;
            }
        }
        true
    };

    let mut res2 = 0;
    'outer: for (s,d) in &sensor_distance {
        for dx in 0..d+2 {
            let dy = (d+1)-dx;
            for (signx, signy) in [(-1,-1),(-1,1),(1,-1),(1,1)] {
                let x = s[0]+(dx*signx);
                let y = s[1]+(dy*signy);
                if 0 <= x && x <= max_range &&  0 <= y && y <= max_range {
                    if valid(x,y,&sensor_distance) {
                        res2 = x * 4_000_000 + y;
                        break 'outer;
                    }
                }
            }
        }
    }

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", res2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
