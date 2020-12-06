use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day05_20_test.txt" } else { "./input/day05_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let mut boardingcards: Vec<i16> = vec![];
    for line in &input {
        let card = line.chars().fold(0, |score, c| match c {
            'B' | 'R' => (score << 1) + 1,
            'F' | 'L' => (score << 1),
            _ => score,
        });
        boardingcards.push(card);
    }
    boardingcards.sort();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 =  boardingcards.iter().last().unwrap();

    let after1 = Instant::now();
    if print_result { println!("Part 1: {}", res1); }

    let start2 = Instant::now();

    let res2 = boardingcards
        .iter()
        .enumerate()
        .take(boardingcards.len()-1)
        .skip(1)
        .filter(|(i,_)| boardingcards[i-1]+2 != boardingcards[i+1])
        .map(|(_,v)| *v)
        .collect::<Vec<i16>>()
        .first()
        .unwrap() + 1;

    let after2 = Instant::now();
    if print_result { println!("Part 2: {}", res2); }

    (after0.duration_since(start0).as_nanos(), after1.duration_since(start1).as_nanos(), after2.duration_since(start2).as_nanos())
}
