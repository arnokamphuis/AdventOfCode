use super::tools;
use std::time::Instant;

type Grid = Vec<char>;

pub fn set_seat(x: i16, y: i16, _: i16, width: i16, grid: &mut Grid, c: char) {
    grid[(y * width + x) as usize] = c;
}

pub fn get_seat(x: i16, y: i16, height: i16, width: i16, grid: &Grid) -> char {
    if !(x < 0 || x >= width || y < 0 || y >= height) {
        let index = y * width + x;
        if index >= 0 && index < (grid.len() as i16) {
            return grid[index as usize];
        }
    }
    '.'
}

#[allow(dead_code)]
pub fn print_map(height: i16, width: i16, grid: &Grid) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", get_seat(x, y, width, height, &grid));
        }
        println!("");
    }
}

pub fn step(grid: &Grid, height: i16, width: i16, queen: bool) -> (bool, Grid) {
    let mut not_changed = true;
    let mut new_grid = grid.clone();
    for y in 0..height {
        for x in 0..width {
            let mut occ_count = 0;
            for dy in -1..2 {
                for dx in -1..2 {
                    if !(dx == 0 && dy == 0) {
                        let mut step_count = 1;
                        loop {
                            if x + step_count * dx < 0
                                || x + step_count * dx >= width
                                || y + step_count * dy < 0
                                || y + step_count * dy >= height
                            {
                                break;
                            }

                            let next_seat = get_seat(
                                x + step_count * dx,
                                y + step_count * dy,
                                height,
                                width,
                                grid,
                            );

                            if next_seat != '.' {
                                if next_seat == '#' {
                                    occ_count += 1;
                                }
                                break;
                            }

                            step_count += 1;

                            if !queen {
                                break;
                            }
                        }
                    }
                }
            }

            let seat = get_seat(x, y, height, width, grid);

            if seat == 'L' && occ_count == 0 {
                set_seat(x, y, height, width, &mut new_grid, '#');
                not_changed = false;
            }
            if seat == '#' && occ_count >= if queen { 5 } else { 4 } {
                set_seat(x, y, height, width, &mut new_grid, 'L');
                not_changed = false;
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
    let height = input.len() as i16;
    let width = input[0].len() as i16;

    let mut grid: Grid = vec![];
    for line in input {
        for c in line.chars() {
            grid.push(c);
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut result = (false, grid.clone());
    while !result.0 {
        result = step(&result.1, height, width, false);
    }

    let res1 = result
        .1
        .iter()
        .fold(0, |acc, v| acc + if *v == '#' { 1 } else { 0 });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    result = (false, grid.clone());
    while !result.0 {
        result = step(&result.1, height, width, true);
    }

    let res2 = result
        .1
        .iter()
        .fold(0, |acc, v| acc + if *v == '#' { 1 } else { 0 });

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
