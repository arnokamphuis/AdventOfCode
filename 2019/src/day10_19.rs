use super::tools;
use num;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

fn determine_angle(station: (u32, u32), to: (u32, u32)) -> (f64, f64) {
    let deltax: f64 = to.0 as f64 - station.0 as f64;
    let deltay: f64 = to.1 as f64 - station.1 as f64;
    let mut angle: f64 = deltax.atan2(deltay);

    if deltay < 0.0 {
        if deltax >= 0.0 {
            angle += std::f64::consts::PI;
        } else {
            angle -= std::f64::consts::PI;
        }
    }

    angle = -angle / 2.0;

    (angle, deltax.powf(2.0) + deltay.powf(2.0))
}

fn check_line(
    from: (u32, u32),
    to: (u32, u32),
    field: &HashMap<(u32, u32), bool>,
    width: usize,
    height: usize,
) -> bool {
    let deltax: i64 = to.0 as i64 - from.0 as i64;
    let deltay: i64 = to.1 as i64 - from.1 as i64;

    let gcd = num::integer::gcd(deltax, deltay) as i64;

    if gcd == 1 {
        true
    } else {
        let stepx: i64 = deltax as i64 / gcd;
        let stepy: i64 = deltay as i64 / gcd;

        let mut x: i64 = from.0 as i64;
        let mut y: i64 = from.1 as i64;

        let mut correct = true;
        loop {
            x += stepx;
            y += stepy;

            if x < 0 || y < 0 || x >= width as i64 || y >= height as i64 {
                break;
            }

            if (x as u32, y as u32) == to {
                break;
            }

            if field[&(x as u32, y as u32)] {
                correct = false;
                break;
            }
        }
        correct
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 10 of 2019");

    let start0 = Instant::now();

    // let input_file = "./input/day10_19_test.txt";
    let input_file = "./input/day10_19_real.txt";
    let input = tools::get_input(String::from(input_file));

    let height = input.len();
    let width = input[0].len();

    let mut field: HashMap<(u32, u32), bool> = HashMap::new();
    input.iter().enumerate().for_each(|(i, r)| {
        r.chars().enumerate().for_each(|(j, a)| {
            field.insert((j as u32, i as u32), a == '#');
        });
    });

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let mut max_count = std::u32::MIN;
    let mut station = (std::u32::MAX, std::u32::MAX);

    field.iter().filter(|p| *((*p).1)).for_each(|p| {
        let mut count = 0;
        field
            .iter()
            .filter(|t| *((*t).1) && *p.0 != *t.0)
            .for_each(|t| {
                if check_line(*p.0, *t.0, &field, width, height) {
                    count += 1;
                }
            });
        if max_count < count {
            station = *p.0;
            max_count = count;
        }
    });

    let after1 = Instant::now();
    println!(
        "Part 1: {} at {:?}, in {:?}",
        max_count,
        station,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    let mut targets: Vec<(f64, f64, u32, u32)> = vec![];

    field.iter().filter(|p| *((*p).1)).for_each(|p| {
        let angle_dist = determine_angle(station, *p.0);

        targets.push((angle_dist.0, angle_dist.1, (*p.0).0, (*p.0).1));
    });

    targets.sort_by(|&(a1, b1, _, _), &(a2, b2, _, _)| {
        if a2 > a1 {
            Ordering::Less
        } else if a2 == a1 && b2 > b1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut shot_asteroids: HashMap<(u32, u32), u32> = HashMap::new();

    let mut index: usize = 0;
    let mut shot = 1;
    let mut destroyed = targets.remove(index);
    shot_asteroids.insert((destroyed.2, destroyed.3), shot);
    loop {
        while targets[index].0 == destroyed.0 {
            index += 1;
            index = index % targets.len();
        }

        destroyed = targets.remove(index);
        shot += 1;
        shot_asteroids.insert((destroyed.2, destroyed.3), shot);

        if shot == 200 {
            break;
        }

        if targets.len() == 0 {
            break;
        }
        index = index % targets.len();
    }

    let result2 = destroyed.2 * 100 + destroyed.3;

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        result2,
        after2.duration_since(start2)
    );
}
