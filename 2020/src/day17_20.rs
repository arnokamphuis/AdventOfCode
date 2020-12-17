use super::tools;
use std::time::Instant;
use std::collections::HashSet;

fn step(cubes_on: &HashSet<[i16; 4]>, part: usize) -> HashSet<[i16; 4]> {
    let mut new_on: HashSet<[i16; 4]> = HashSet::new();

    let range = |dim: usize, on: &HashSet<[i16;4]>| -> std::ops::Range<i16> {
        let r = on.iter().fold((100,-100), |acc, v| 
            (std::cmp::min(acc.0,v[dim]),std::cmp::max(acc.1,v[dim])));
        std::ops::Range{ start: r.0-1 , end: r.1+2}
    };

    for x in range(0, cubes_on) {
        for y in range(1, cubes_on) {
            for z in range(2, cubes_on) {
                for w in if part == 1 { 0..1 } else { range(3, cubes_on) } {
                    let pos = [x,y,z,w];
                    let mut count: usize = 0;
                    'outer: for dx in -1..2i16 {
                        for dy in -1..2i16 {
                            for dz in -1..2i16 {
                                for dw in if part == 1 { 0..1i16 } else { -1..2i16 } {
                                    if dx!=0 || dy!=0 || dz!=0 || dw!=0 {
                                        if cubes_on.contains(&[x+dx, y+dy, z+dz, w+dw]) {
                                            count += 1;
                                            if count > 3 {
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let current = cubes_on.contains(&pos);
                    if (current && (count == 2 || count == 3)) || (!current && count == 3) {
                        new_on.insert(pos.clone());
                    }
                }
            }        
        }
    }
    new_on
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day17_20_test.txt"
    } else {
        "./input/day17_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut grid: HashSet<[i16;4]> = HashSet::new();

    input.iter().enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c=='#' { grid.insert([x as i16,y as i16,0,0]); }
        })
    });
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut cubes_on1 = grid.clone();
    (0..6).for_each(|_| {
        cubes_on1 = step(&cubes_on1, 1);
    });
    let res1 = cubes_on1.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut cubes_on2 = grid.clone();
    (0..6).for_each(|_| {
        cubes_on2 = step(&cubes_on2, 2);
    });

    let res2 = cubes_on2.len();

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
