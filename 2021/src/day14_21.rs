use super::tools;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_21_test.txt"
    } else {
        "./input/day14_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let polymer: Vec<char> = input[0].chars().collect();
    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    let mut chars: BTreeSet<char> = BTreeSet::new();
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    let mut emptycounts: HashMap<(char, char), usize> = HashMap::new();

    input.iter().skip(2).for_each(|line| {
        let mut tokens = line.split(" -> ");
        let from = tokens.next().unwrap().chars().collect::<Vec<char>>();
        let to: char = tokens.next().unwrap().chars().next().unwrap();
        chars.insert(from[0]);
        chars.insert(from[1]);
        chars.insert(to);
        rules.insert(from, to);
    });

    for c1 in &chars {
        for c2 in &chars {
            counts.insert((*c1, *c2), 0);
            emptycounts.insert((*c1, *c2), 0);
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    polymer.windows(2).for_each(|p| {
        if let Some(mc) = counts.get_mut(&(p[0], p[1])) {
            *mc += 1;
        }
    });

    let last: char = polymer.iter().last().unwrap().clone();
    (0..10).for_each(|_| {
        counts = rules
            .iter()
            .fold(emptycounts.clone(), |mut newcounts, (pair, &newc)| {
                let cc = counts[&(pair[0], pair[1])];
                *newcounts.get_mut(&(pair[0], newc)).unwrap() += cc;
                *newcounts.get_mut(&(newc, pair[1])).unwrap() += cc;
                newcounts
            });
    });

    let mut newcounts = counts.iter().map(|c| (c.0 .0, c.1)).fold(
        HashMap::new(),
        |mut map: HashMap<char, usize>, (c, count)| {
            if let Some(mc) = map.get_mut(&c) {
                *mc += *count;
            } else {
                map.insert(c, *count);
            }
            map
        },
    );
    if let Some(c) = newcounts.get_mut(&last) {
        *c += 1;
    }

    let res1 = newcounts.iter().map(|(_, c)| *c).max().unwrap()
        - newcounts.iter().map(|(_, c)| *c).min().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    (10..40).for_each(|_| {
        counts = rules
            .iter()
            .fold(emptycounts.clone(), |mut newcounts, (pair, &newc)| {
                let cc = counts[&(pair[0], pair[1])];
                *newcounts.get_mut(&(pair[0], newc)).unwrap() += cc;
                *newcounts.get_mut(&(newc, pair[1])).unwrap() += cc;
                newcounts
            });
    });

    let mut newcounts = counts.iter().map(|c| (c.0 .0, c.1)).fold(
        HashMap::new(),
        |mut map: HashMap<char, usize>, (c, count)| {
            if let Some(mc) = map.get_mut(&c) {
                *mc += *count;
            } else {
                map.insert(c, *count);
            }
            map
        },
    );
    if let Some(c) = newcounts.get_mut(&last) {
        *c += 1;
    }

    let res2 = newcounts.iter().map(|(_, c)| *c).max().unwrap()
        - newcounts.iter().map(|(_, c)| *c).min().unwrap();

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
