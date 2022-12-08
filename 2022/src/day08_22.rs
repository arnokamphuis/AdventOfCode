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
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut count = 2 * w + 2 * h - 4;
    for y in 1..h-1 {
        for x in 1..w-1 {
            let mut visible = vec![true;4];
            for dx in (0..x).rev() { visible[0] &= grid[y][x] > grid[y][dx]; }
            for dx in  x+1..w      { visible[1] &= grid[y][x] > grid[y][dx]; }
            for dy in (0..y).rev() { visible[2] &= grid[y][x] > grid[dy][x]; }
            for dy in  y+1..h      { visible[3] &= grid[y][x] > grid[dy][x]; }
            if visible.iter().fold(false, |acc, &v| acc || v) { count+=1 };
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", count);
    }

    let start2 = Instant::now();

    let score = | x: usize, y: usize | -> i64 {
        let mut scores = vec![0i64;4];
        for dx in (0..x).rev() { scores[0] += 1; if grid[y][x] <= grid[y][dx] { break; } }
        for dx in  x+1..w      { scores[1] += 1; if grid[y][x] <= grid[y][dx] { break; } }
        for dy in (0..y).rev() { scores[2] += 1; if grid[y][x] <= grid[dy][x] { break; } }
        for dy in  y+1..h      { scores[3] += 1; if grid[y][x] <= grid[dy][x] { break; } }
        scores[0] * scores[1] * scores[2] * scores[3]
    };

    let mut all_scores = vec![];
    (0..h).for_each(|y| (0..w).for_each(|x| all_scores.push(score(x,y)) ));
    let res2: i64 = *all_scores.iter().max().unwrap();

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
