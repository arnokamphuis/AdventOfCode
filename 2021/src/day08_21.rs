use super::tools;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_21_test.txt"
    } else {
        "./input/day08_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut res1 = 0;
    let mut firsts: Vec<Vec<BTreeSet<char>>> = vec![];
    let mut seconds: Vec<Vec<BTreeSet<char>>> = vec![];
    input.iter().for_each(|line| {
        let twoparts: Vec<String> = line.split(" | ").fold(vec![], |mut list: Vec<String>, s| {
            list.push(String::from(s));
            list
        });
        let first: Vec<BTreeSet<char>> =
            twoparts[0]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        firsts.push(first);
        let second: Vec<BTreeSet<char>> =
            twoparts[1]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        seconds.push(second);
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    seconds.iter().for_each(|second| {
        let digits = second
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .map(|v| v.clone())
            .collect::<Vec<BTreeSet<char>>>();
        res1 += digits.iter().count();
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut res2 = 0;
    firsts
        .iter()
        .zip(seconds.iter())
        .for_each(|(first, second)| {
            let mut knownchars: HashMap<char, char> = HashMap::new();
            let mut knowndigits: HashMap<usize, BTreeSet<char>> = HashMap::new();

            let mut d = first
                .iter()
                .filter(|&s| s.len() == 2)
                .map(|s| s.clone())
                .collect::<Vec<BTreeSet<char>>>();
            if d.len() > 0 {
                knowndigits.insert(1, d[0].clone());
            }
            d = first
                .iter()
                .filter(|&s| s.len() == 3)
                .map(|s| s.clone())
                .collect::<Vec<BTreeSet<char>>>();
            if d.len() > 0 {
                knowndigits.insert(7, d[0].clone());
            }
            d = first
                .iter()
                .filter(|&s| s.len() == 4)
                .map(|s| s.clone())
                .collect::<Vec<BTreeSet<char>>>();
            if d.len() > 0 {
                knowndigits.insert(4, d[0].clone());
            }
            d = first
                .iter()
                .filter(|&s| s.len() == 7)
                .map(|s| s.clone())
                .collect::<Vec<BTreeSet<char>>>();
            if d.len() > 0 {
                knowndigits.insert(8, d[0].clone());
            }

            let mut a = knowndigits[&7].clone();
            a.retain(|c| !knowndigits[&1].contains(c));
            println!("{:?}", a);
            assert!(a.len() == 1);
            a.iter().for_each(|&c| {
                knownchars.insert('a', c);
            });

            println!("{:?}", knownchars);
        });

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
