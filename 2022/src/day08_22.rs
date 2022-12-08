use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_22_test.txt"
    } else {
        "./input/day08_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let h = input.len();
    let w = input[0].len();

    let grid: Vec<Vec<u8>> = input.iter().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>()
    }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut count = 2 * w + 2 * h - 4;
    for y in 1..h-1 {
        for x in 1..w-1 {
            let mut visible = true;
            for dx in 0..x   { visible = visible && (grid[y][x] > grid[y][dx]) };
            if visible { count += 1; }
            else {
                visible = true;
                for dx in x+1..w { visible = visible && (grid[y][x] > grid[y][dx]) };
                if visible { count += 1; }
                else {
                    visible = true;
                    for dy in 0..y   { visible = visible && (grid[y][x] > grid[dy][x]) };
                    if visible { count += 1; }
                    else {
                        visible = true;
                        for dy in y+1..h { visible = visible && (grid[y][x] > grid[dy][x]) };
                        if visible { count += 1; }
                    }
                }
            }
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", count);
    }

    let start2 = Instant::now();

    let score = | x: usize, y: usize | -> usize {
        let mut scores = vec![0;4];
        for dx in 0..x   { if grid[y][x] >= grid[y][dx] { scores[0] = x-dx; break; } };
        for dx in x+1..w { if grid[y][x] >= grid[y][dx] { scores[1] = dx-x; break; } };
        for dy in 0..y   { if grid[y][x] >= grid[dy][x] { scores[2] = y-dy; break; } };
        for dy in y+1..h { if grid[y][x] >= grid[dy][x] { scores[3] = dy-y; break; } };
        println!("{} {} {:?} -> {}", x, y, scores, scores[0] * scores[1] * scores[2] * scores[3]);
        scores[0] * scores[1] * scores[2] * scores[3]
    };

    let mut all_scores = vec![];
    for y in 0..h {
        for x in 0..w {
            all_scores.push(score(x,y));
        }
    }

    let res2: usize = *all_scores.iter().max().unwrap();

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
