use super::tools;
use std::time::Instant;
use tools::Image;

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

#[allow(dead_code)]
pub fn image_map(part: usize, count: usize, height: i16, width: i16, grid: &Grid) {
    let mut img = Image::new(width as usize, height as usize, 4);
    for y in 0..height {
        for x in 0..width {
            img.set_pixel(x as usize, y as usize, match get_seat(x, y, height, width, &grid) {
                '#' => (255,255,255,255),
                'L' => (200,200,200,255),
                // '.' => (0,255,0,255),
                _ => (0,0,0,255),
            });
        }
    }
    img.save_png(&format!("images/seats-{}-{:05}.png", part, count));
}

pub fn step(grid: &Grid, height: i16, width: i16, queen: bool) -> (bool, Grid) {
    let mut not_changed = true;
    let mut new_grid = grid.clone();
    for y in 0..height {
        for x in 0..width {
            let seat = get_seat(x, y, height, width, grid);
            if seat != '.' {
                let mut occ_count = 0;
                'outer: for dy in -1..2 {
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
                        if occ_count >= 5 {
                            break 'outer;
                        }
                    }
                }

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

    // let mut count = 0;
    let mut result = (false, grid.clone());
    // image_map(1, count, height, width, &result.1);
    while !result.0 {
        result = step(&result.1, height, width, false);
        // count += 1;
        // image_map(0, count, height, width, &result.1);
    }

    // for i in 0..10 {
    //     count+=1;
    //     image_map(0, count, height, width, &result.1);
    // }

    let res1 = result
        .1
        .iter()
        .fold(0, |acc, v| acc + if *v == '#' { 1 } else { 0 });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    // count = 0;
    result = (false, grid.clone());
    // image_map(2, count, height, width, &result.1);
    while !result.0 {
        result = step(&result.1, height, width, true);
        // count += 1;
        // image_map(0, count, height, width, &result.1);
    }

    // for i in 0..10 {
    //     count+=1;
    //     image_map(0, count, height, width, &result.1);
    // }

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
