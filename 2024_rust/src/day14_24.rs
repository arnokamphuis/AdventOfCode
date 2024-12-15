use super::tools;
use std::time::Instant;
use regex::Regex;
use itertools::Itertools;
#[cfg(feature = "make_movie")]
use crate::tools::Image;

fn calculate_position(time: i32, (c,r): (i32, i32), ((x,y),(vx,vy)): ((i32,i32),(i32,i32))) -> (i32,i32) {
    ((x + time*vx).rem_euclid(c), (y + time*vy).rem_euclid(r))
}

fn get_quadrants(pos:(i32,i32), size: (i32, i32)) -> usize {
    let mid = (size.0/2, size.1/2);
    match pos {
        (x,y) if x < mid.0 && y < mid.1 => 1,
        (x,y) if x > mid.0 && y < mid.1 => 2,
        (x,y) if x < mid.0 && y > mid.1 => 3,
        (x,y) if x > mid.0 && y > mid.1 => 4,
        _ => 0
    }
}

fn get_strong_connected_robots(positions: &Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    let dirs = vec![(0,1), (0,-1), (1,0), (-1,0)];
    positions.iter().filter(|(x,y)| dirs.iter().all(|(dx,dy)| positions.contains(&(x+dx, y+dy)))).map(|&p| p).collect_vec()
}


#[cfg(feature = "make_movie")]
fn make_frame(robots: &Vec<((i32,i32),(i32,i32))>, (c,r): (i32, i32), time: i32, frame_id: i32) {
    let mut img: Image = Image::new(c as usize, r as usize, 4);
    img.clear((0, 0, 0, 255));
    let positions = robots.iter().map(|p| calculate_position(time, (c,r), *p)).collect_vec();
    for rr in 0..r {
        for cc in 0..c {
            if positions.contains(&(cc, rr)) {
                img.set_pixel(cc as usize, rr as usize, (255, 255, 255, 255));
            }
        }
    }
    img.save_png(&format!("images/day14_24_{:06}.png", frame_id));
}

#[cfg(feature = "make_movie")]
fn make_movie(robots: &Vec<((i32,i32),(i32,i32))>, (c,r): (i32, i32), max_time: i32) {
    (1..=max_time).for_each(|time| {
        make_frame(robots, (c,r), time, time);
    });
    (1..=50).for_each(|time| {
        make_frame(robots, (c,r), max_time, max_time + time);
    });
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14-test.txt"
    } else {
        "./input/day14-real.txt"
    };

    let input = tools::get_input(String::from(input_file));

    let re = Regex::new(r"[-+]?\d+").unwrap();
    let robots: Vec<((i32,i32),(i32,i32))> = input
        .iter()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .into_iter()
                .collect_tuple()
                .unwrap()
        })
        .map(|(x,y,vx,vy)| ((x,y),(vx,vy)))
        .collect_vec();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let (c,r) = if !real { (11, 7) } else { (101, 103) };

    let res1 = robots
        .iter()
        .map(|p| calculate_position(100, (c,r), *p))
        .map(|p| get_quadrants(p, (c,r)))
        .fold(vec![0;5], |mut acc, x| { acc[x] += 1; acc })
        .iter().skip(1)
        .product::<usize>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();


    let mut time = 1;
    loop {
        if get_strong_connected_robots(&robots.iter().map(|p| calculate_position(time, (c,r), *p)).collect_vec()).len() > 5 {
            break;
        }
        time += 1;
    }

    let res2 = time;

    #[cfg(feature = "make_movie")]
    make_movie(&robots, (c,r), res2);

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
