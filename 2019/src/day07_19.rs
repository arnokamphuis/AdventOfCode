use super::tools;
use permutohedron::heap_recursive;
use std::collections::BTreeMap;
use std::time::Instant;

use super::intcode::IntCodeComputer;

fn run_amplifiers(phases: &Vec<i64>, commands: &BTreeMap<i64, i64>, part: usize) -> i64 {
    const AMPLIFIERCOUNT: usize = 5;
    let mut result = -1;
    let mut amplifiers: Vec<IntCodeComputer> =
        vec![IntCodeComputer::new(&commands); AMPLIFIERCOUNT];

    for i in 0..AMPLIFIERCOUNT {
        amplifiers[i].add_input(phases[i]);
    }

    amplifiers[0].add_input(0);

    let mut running = true;
    while running {
        let mut finished: bool = false;
        for i in 0..AMPLIFIERCOUNT {
            finished = amplifiers[i].run();

            while let Some(out0) = amplifiers[i].get_output() {
                amplifiers[(i + 1) % 5].add_input(out0);
                result = out0
            }
        }

        if finished || part == 1 {
            running = false;
        }
    }
    result
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 7 of 2019");

    // let input_file = "./input/day07_19_test.txt";
    let input_file = "./input/day07_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let line = &input[0];
    let command_strings: Vec<&str> = line.split(",").collect();
    let mut commands: BTreeMap<i64, i64> = BTreeMap::new();
    command_strings
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .for_each(|(i, c)| {
            commands.insert(i as i64, c);
        });

    let start1 = Instant::now();

    let mut part = 1;
    let mut data1 = [0, 1, 2, 3, 4];
    let mut permutations1 = Vec::new();
    heap_recursive(&mut data1, |permutation| {
        permutations1.push(permutation.to_vec())
    });

    let mut highest_signal: i64 = std::i64::MIN;
    permutations1.iter().for_each(|p| {
        let output = run_amplifiers(p, &commands, part);
        if output > highest_signal {
            highest_signal = output;
        }
    });

    let after1 = Instant::now();
    println!(
        "Part 1: {}, in {:?}",
        highest_signal,
        after1.duration_since(start1)
    );

    let start2 = Instant::now();

    part = 2;
    let mut data2 = [5, 6, 7, 8, 9];
    let mut permutations2 = Vec::new();
    heap_recursive(&mut data2, |permutation| {
        permutations2.push(permutation.to_vec())
    });

    highest_signal = std::i64::MIN;
    permutations2.iter().for_each(|p| {
        let output = run_amplifiers(p, &commands, part);
        if output > highest_signal {
            highest_signal = output;
        }
    });

    let after2 = Instant::now();
    println!(
        "Part 2: {}, in {:?}",
        highest_signal,
        after2.duration_since(start2)
    );
}
