use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run() {
    println!("Day 05 of 2020");

    let start0 = Instant::now();

    // let input_file: &str = "./input/day05_20_test.txt";
    let input_file: &str = "./input/day05_20_real.txt";
    let input = tools::get_input(String::from(input_file));

    let mut boardingcards: Vec<i64> = vec![];
    for line in &input {
        let card = line.chars().fold(0, |score, c| match c {
            'B' => score * 2 + 1,
            'F' => score * 2,
            'R' => score * 2 + 1,
            'L' => score * 2,
            _ => score,
        });
        boardingcards.push(card);
    }

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let res1 =  boardingcards.iter().max().unwrap();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", res1, after1.duration_since(start1));

    let start2 = Instant::now();

    let mut res2: i64 = 0;

    boardingcards.sort();
    for i in 1..boardingcards.len() {
        if boardingcards[i-1]+1 != boardingcards[i] {
            res2 = boardingcards[i-1]+1;
        }
    }

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", res2, after2.duration_since(start2));
}
