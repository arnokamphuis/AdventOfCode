use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day25_21_test.txt"
    } else {
        "./input/day25_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut field = input.iter().fold(vec![], |mut v, line| {
        v.push(line.chars().collect::<Vec<char>>()); v
    });

    let dir_step = | prev: &Vec<Vec<char>>, dir: char | -> Vec<Vec<char>> {
        let h = prev.len();
        let w = prev[0].len();
        let mut new = vec![vec![' ';w];h];
        for i in 1..=h {
            for j in 1..=w {
                let curr_i = i % h;
                let curr_j = j % w;
                let prev_i = (i - (dir == 'v') as usize) % h;
                let prev_j = (j - (dir == '>') as usize) % w;
                if prev[prev_i][prev_j] == dir && prev[curr_i][curr_j] == '.' {
                    new[curr_i][curr_j] = dir;
                    new[prev_i][prev_j] = '.';
                } else {
                    if new[prev_i][prev_j] == ' ' {
                        new[prev_i][prev_j] = prev[prev_i][prev_j];
                    }
                }
            }
        }
        new
    };

    let step = | prev: &Vec<Vec<char>> | -> Vec<Vec<char>> {
        dir_step(&dir_step(prev, '>'), 'v')
    };
    
    // let print = | f: &Vec<Vec<char>> | {
    //     f.iter().for_each(|v| {
    //         v.iter().for_each(|c| print!("{}",c));
    //         println!("");
    //     });
    //     println!("");
    // };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut step_counter = 0;
    loop {
        step_counter += 1;
        let new_field = step(&field);

        if new_field == field {
            break;
        }
        field = new_field;
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", step_counter);
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
