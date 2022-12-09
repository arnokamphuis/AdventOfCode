use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn play_rope(rope_length: usize, visited: &mut HashSet<[i32;2]>, commands: &Vec<String>) {
    let mut rope: Vec<[i32;2]> = vec![[0,0];rope_length];
    visited.clear();
    visited.insert([0,0]);

    let dirs: HashMap<_, _> = [
        ('R', ( 1i32, 0i32)),
        ('L', (-1i32, 0i32)),
        ('U', ( 0i32, 1i32)),
        ('D', ( 0i32,-1i32)),
    ].into_iter().collect();

    let update_knot = | h: &[i32;2], t: &mut [i32;2] | {
        let ht = [h[0] - t[0], h[1] - t[1]]; 
        if ht[0].abs() == 2 && ht[1] == 0 {
            t[0] += ht[0]/ht[0].abs();
        } else if ht[1].abs() == 2 && ht[0] == 0 {
            t[1] += ht[1]/ht[1].abs();
        } else if ht[0].abs() + ht[1].abs() == 3 {
            if ht[0] != 0 { t[0] += ht[0]/ht[0].abs(); }
            if ht[1] != 0 { t[1] += ht[1]/ht[1].abs(); }
        } else if ht[0].abs() == 2 && ht[1].abs() == 2 {
            t[0] += ht[0]/ht[0].abs();
            t[1] += ht[1]/ht[1].abs();
        }
    };

    commands.iter().for_each(|line| {
        let directions: Vec<&str> = line.split_whitespace().collect();
        let d = directions[0].chars().nth(0).unwrap();
        let n = directions[1].parse::<i32>().unwrap();

        (0..n).for_each(|_| {
            
            rope[0][0] += dirs[&d].0;
            rope[0][1] += dirs[&d].1;
            for i in 1..rope_length {
                let h = rope[i-1].clone();
                update_knot(&h, &mut rope[i]);                
            }
            visited.insert(rope[rope_length-1]);
        });
    });
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_22_test.txt"
    } else {
        "./input/day09_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut visited: HashSet<[i32;2]> = HashSet::new();

    let after0 = Instant::now();

    let start1 = Instant::now();

    play_rope(2, &mut visited, &input);
    let res1 = visited.len();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    play_rope(10, &mut visited, &input);
    let res2 = visited.len();

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
