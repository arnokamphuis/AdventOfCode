use super::tools;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

fn solve(game: Vec<usize>, part2: bool) -> usize {
    let (a,b,c,d,mut e,mut f): (f64,f64,f64,f64,f64,f64) = game.iter().map(|v| *v as f64).collect_tuple().unwrap();
    if part2 {
        e += 10000000000000.0;
        f += 10000000000000.0;
    }
    let x = (c*f - d*e) / (b*c - a*d);
    let y = (e - a*x) / c;
    
    if x%1.0 == 0.0 && y%1.0 == 0.0 {
        return 3*(x as usize) + (y as usize)
    }
    return 0
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day13-test.txt"
    } else {
        "./input/day13-real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let re = Regex::new(r"\d+").unwrap();
    let games: Vec<_> = input
        .chunks(4)
        .into_iter()
        .map(|c| {
            re.find_iter(&c.join(" "))
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .into_iter()
                .collect_vec()
        })
        .collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = games.iter().map(|g| solve(g.clone(), false)).sum::<usize>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = games.iter().map(|g| solve(g.clone(), true)).sum::<usize>();

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
