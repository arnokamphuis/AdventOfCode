use std::time::{Instant};
use super::tools;
use super::intcode::IntCodeComputer;
use super::intcode::get_commands_from_line;

#[allow(dead_code)]
fn from_ascii(c: char) -> i64 {
    (c as u8) as i64
}

fn to_ascii(c: i64) -> char {
    (c as u8) as char
}

#[allow(dead_code)]
pub fn run() {
    println!("Day 21 of 2019");

    let start0 = Instant::now();

    let input_file = "./input/day21_19_real.txt";
    let input = tools::get_input(String::from(input_file));
    let mut commands = get_commands_from_line(&input[0]);

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let droidscript1 = tools::get_input(String::from("./input/day21_19_droidscript1.txt"));
    let mut droidcomp1 = IntCodeComputer::new(&commands, false);
    droidcomp1.run();
    while let Some(out) = droidcomp1.get_output() {
        print!("{}", to_ascii(out));
    }

    for ds in droidscript1 {
        for c in ds.chars() {
            print!("{} ", from_ascii(c));
            droidcomp1.add_input(from_ascii(c));
        }
        print!("{} ", 10);
        droidcomp1.add_input(10);
    }

    println!("");
    while !droidcomp1.run() {
    }

    let mut res1 = -1;
    while let Some(out) = droidcomp1.get_output() {
        if out > 255 {
            res1 = out;
        } else {
            print!("{}", to_ascii(out));
        }
    }    

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let droidscript2 = tools::get_input(String::from("./input/day21_19_droidscript2.txt"));
    let mut droidcomp2 = IntCodeComputer::new(&commands, false);
    droidcomp2.run();
    while let Some(out) = droidcomp2.get_output() {
        print!("{}", to_ascii(out));
    }

    for ds in droidscript2 {
        for c in ds.chars() {
            print!("{} ", from_ascii(c));
            droidcomp2.add_input(from_ascii(c));
        }
        print!("{} ", 10);
        droidcomp2.add_input(10);
    }

    println!("");
    while !droidcomp2.run() {
    }

    let mut res2 = -1;
    while let Some(out) = droidcomp2.get_output() {
        if out > 255 {
            res2 = out;
        } else {
            print!("{}", to_ascii(out));
        }
    }    

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}
