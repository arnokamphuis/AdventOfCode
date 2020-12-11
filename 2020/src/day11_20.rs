use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;

type Grid = BTreeMap<(i32,i32), char>;

pub fn step(grid: &Grid, height: i32, width: i32) -> (bool, Grid) {
    let mut not_changed = true;
    let mut new_grid = grid.clone();
    for y in 0..height {
        for x in 0..width {
            if let Some(seat) = grid.get(&(x,y)) {
                if *seat != '.' {
                    let mut occ_count = 0;
                    let mut free_count = 0;
                    let tl; if let Some(c) = grid.get(&(x-1,y-1)) { tl = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let tm; if let Some(c) = grid.get(&(x-1,y+0)) { tm = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let tr; if let Some(c) = grid.get(&(x-1,y+1)) { tr = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let ml; if let Some(c) = grid.get(&(x+0,y-1)) { ml = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let mr; if let Some(c) = grid.get(&(x+0,y+1)) { mr = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let bl; if let Some(c) = grid.get(&(x+1,y-1)) { bl = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let bm; if let Some(c) = grid.get(&(x+1,y+0)) { bm = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    let br; if let Some(c) = grid.get(&(x+1,y+1)) { br = *c; if *c == '#' { occ_count += 1; } if *c == 'L' { free_count += 1; } }
                    if *seat == 'L' && occ_count == 0 {
                        if let Some(e) = new_grid.get_mut(&(x,y)) {
                            *e = '#';
                            not_changed = false;
                        }
                    }   
                    if *seat == '#' && free_count >= 4 {
                        if let Some(e) = new_grid.get_mut(&(x,y)) {
                            *e = 'L';
                            not_changed = false;
                        }
                    }   
                }
            }
        }
    }
    (not_changed, new_grid)
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day11_20_test.txt"
    } else {
        "./input/day11_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut grid: Grid = BTreeMap::new();
    let mut x = 0;
    let mut y = 0;

    for line in input {
        x = 0;
        for c in line.chars() {
            grid.insert((x,y), c);
            x += 1;
        }
        y += 1;
    }

    println!("{:?}", grid);
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut round_count = 1;
    let mut result = step(&grid,height, width);

    for y in &grid {
        println!("{:?}", y);
    }

    while !result.0 {
        result = step(&result.1, height, width);
        round_count += 1;
    }

    println!("round count {}", round_count);


    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", 0);
    }

    let start2 = Instant::now();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", 0);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
