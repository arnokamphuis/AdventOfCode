use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17_21_test.txt"
    } else {
        "./input/day17_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut tokens = input[0][13..].split(", ");
    let mut x_tokens = tokens.next().unwrap()[2..].split("..");
    let mut y_tokens = tokens.next().unwrap()[2..].split("..");

    let xmin = x_tokens.next().unwrap().parse::<i64>().unwrap();
    let xmax = x_tokens.next().unwrap().parse::<i64>().unwrap();
    let ymin = y_tokens.next().unwrap().parse::<i64>().unwrap();
    let ymax = y_tokens.next().unwrap().parse::<i64>().unwrap();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut y_velocities: Vec<i64> = vec![];
    let mut max_height = 0;
    for y_speed in -100..=100 {
        let mut y: i64   = 0;
        let mut vy: i64  = y_speed;
        let mut max: i64 = y;

        while (y+vy) >= ymin {
            y += vy; vy -= 1;
            max = y.max(max);
        }

        if y >= ymin && y <= ymax {
            y_velocities.push(y_speed);
            max_height = max_height.max(max);
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", max_height);
    }

    let start2 = Instant::now();

    let mut velocities: Vec<(i64,i64)> = vec![];
    for y_speed in y_velocities {
        for x_speed in 0..=xmax {
            let mut y: i64   = 0;
            let mut vy: i64  = y_speed;

            let mut x: i64   = 0;
            let mut vx: i64  = x_speed;

            let check = | x, y | -> bool {
                x >= xmin && x <= xmax && y >= ymin && y <= ymax
            };

            let mut hit = false;
            while y >= ymin && x <= xmax && !hit {
                x += vx; vx = if vx == 0 {vx} else {vx - vx/vx.abs()};
                y += vy; vy -= 1;
                hit |= check(x,y);
            }

            if hit {
                velocities.push((x_speed,y_speed));
            }
    
        }
    } 

    let res2 = velocities.len();

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
