use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day10_21_test.txt"
    } else {
        "./input/day10_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let startchars = ['{', '<', '[', '('];
    let endchars   = ['}', '>', ']', ')'];
    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut scoredlines: Vec<(String, u64)> = input.iter().map(|line| {
        let mut q: Vec<char> = vec![];
        let mut ok = true;
        let mut score = 0;
        line.chars().for_each(|c| {
            if ok {
                if startchars.contains(&c) {
                    q.push(c);
                } else {
                    if let Some(sc)= q.pop() {
                        if startchars.iter().position(|&f| f==sc).unwrap() != endchars.iter().position(|&f| f==c).unwrap() {
                            score += match c {
                                ')' => {     3 },
                                ']' => {    57 },
                                '}' => {  1197 },
                                '>' => { 25137 },
                                _   => {     0 }
                            };
                            ok = false;
                        } 
                    }
                }
            }
        });
        (line.clone(),score)
    }).collect();

    let res1: u64 = scoredlines.iter().map(|(_,score)| score).filter(|&score| *score > 0).sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    scoredlines.retain(|(_,score)| *score == 0);
    let mut scores: Vec<u64> = scoredlines.iter().fold(vec![],|mut scores, (line,_)| {
        let mut q: Vec<char> = vec![];

        line.chars().for_each(|c| { if startchars.contains(&c) { q.push(c); } else { q.pop(); }});
        scores.push(q.iter().rev().fold(0, |s, c| {
            s * 5 + match c {
                '(' => { 1 },
                '[' => { 2 },
                '{' => { 3 },
                '<' => { 4 },
                _   => { 0 }
            }
        }));
        scores
    });

    scores.sort();
    let res2 = scores[ scores.len()/2 ];

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
