use super::tools;
use std::cmp;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05_21_test.txt"
    } else {
        "./input/day05_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut xcoors: Vec<usize> = vec![];
    let mut ycoors: Vec<usize> = vec![];
    let lines: Vec<((usize, usize), (usize, usize))> = input
        .iter()
        .map(|line| {
            let mut tokens = line.split(" -> ");
            let mut fromtokens = tokens.next().unwrap().split(",");
            let f1 = fromtokens.next().unwrap().parse::<usize>().unwrap();
            let f2 = fromtokens.next().unwrap().parse::<usize>().unwrap();
            let mut totokens = tokens.next().unwrap().split(",");
            let t1 = totokens.next().unwrap().parse::<usize>().unwrap();
            let t2 = totokens.next().unwrap().parse::<usize>().unwrap();
            xcoors.push(f1);
            xcoors.push(t1);
            ycoors.push(f2);
            ycoors.push(t2);
            ((f1, f2), (t1, t2))
        })
        .collect();

    let maxx = *xcoors.iter().max().unwrap();
    let maxy = *ycoors.iter().max().unwrap();

    let mut field: Vec<Vec<usize>> = vec![vec![0; maxy + 1]; maxx + 1];

    let hvlines: Vec<((usize, usize), (usize, usize))> = lines
        .iter()
        .filter(|line| line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1)
        .map(|&l| l)
        .collect();
    let diaglines: Vec<((usize, usize), (usize, usize))> = lines
        .iter()
        .filter(|line| line.0 .0 != line.1 .0 && line.0 .1 != line.1 .1)
        .map(|&l| l)
        .collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    for line in hvlines {
        for x in cmp::min(line.0 .0, line.1 .0)..cmp::max(line.0 .0, line.1 .0) + 1 {
            for y in cmp::min(line.0 .1, line.1 .1)..cmp::max(line.0 .1, line.1 .1) + 1 {
                field[x][y] += 1;
            }
        }
    }
    let res1 = field.iter().flatten().filter(|&&v| v > 1).count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    for line in diaglines {
        let delta = line.0 .0 as i32 - line.1 .0 as i32;
        let dirx: i32 = if delta < 0 { -1 } else { 1 };
        let diry: i32 = if line.0 .1 < line.1 .1 { -1 } else { 1 };
        for d in 0i32..delta.abs() + 1 {
            let x = (line.1 .0 as i32 + d * dirx) as usize;
            let y = (line.1 .1 as i32 + d * diry) as usize;
            field[x][y] += 1;
        }
    }
    let res2 = field.iter().flatten().filter(|&&v| v > 1).count();

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
