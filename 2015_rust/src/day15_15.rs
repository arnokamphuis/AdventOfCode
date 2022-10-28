use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone)]
struct TeaSpoons {
    index: usize,
    values: Vec<Vec<usize>>,
}

impl TeaSpoons {
    fn new() -> TeaSpoons {
        let mut ts = TeaSpoons {
            index: 0,
            values: vec![],
        };

        for a in 0..=100 {
            for b in 0..=100-a {
                for c in 0..=100-a-b {
                    let d = 100 - a - b - c;
                    ts.values.push(vec![a,b,c,d]);
                }
            }
        }
        ts
    }
}

impl Iterator for TeaSpoons {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index == self.values.len() {
            None
        } else {
            Some(self.values[self.index-1].clone())
        }
    }
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day15_15_test.txt"
    } else {
        "./input/day15_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let ingredients: HashMap<String, Vec<i32>> = input.iter().map(|line| {
        let ingr = line.split(": ").map(|s| s.to_string()).collect::<Vec<String>>();
        let props = ingr[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        (ingr[0].to_string(), props.iter().map(|p| {
            p.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()[1].parse::<i32>().unwrap()
        }).collect())
    }).collect();

    let ts1: TeaSpoons = TeaSpoons::new();
    let ts2 = ts1.clone();

    let after0 = Instant::now();

    let start1 = Instant::now();
    let res1 = ts1.map(|current_spoons| {
        current_spoons
            .iter()
            .map(|&cs| cs)
            .zip(
                ingredients.iter().map(|(_,props)| props.to_vec())
            )
            .map(|(f,v)| v.iter().map(|vi| f as i32 * vi).collect::<Vec<i32>>())
            .fold(vec![0;5], |mut sum, val| { for i in 0..5 {sum[i] += val[i]; } sum })
        })
        .map(|vec| {let mut prod = 1; for i in 0..4 { prod *= vec[i].max(0); } prod })
        .max().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = ts2.map(|current_spoons| {
        current_spoons
            .iter()
            .map(|&cs| cs)
            .zip(
                ingredients.iter().map(|(_,props)| props.to_vec())
            )
            .map(|(f,v)| v.iter().map(|vi| f as i32 * vi).collect::<Vec<i32>>())
            .fold(vec![0;5], |mut sum, val| { for i in 0..5 {sum[i] += val[i]; } sum })
        })
        .filter(|vec| vec[4] == 500)
        .map(|vec| {let mut prod = 1; for i in 0..4 { prod *= vec[i].max(0); } prod })
        .max().unwrap();

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
