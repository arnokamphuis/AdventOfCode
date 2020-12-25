use super::tools;
use std::collections::BTreeSet;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day06_20_test.txt"
    } else {
        "./input/day06_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut groups: Vec<Vec<BTreeSet<char>>> = vec![];
    let mut group: Vec<BTreeSet<char>> = vec![];

    for line in &input {
        let answers = line.chars().fold(BTreeSet::new(), |mut set, c| {
            set.insert(c);
            set
        });

        if answers.len() == 0 {
            groups.push(group.clone());
            group = vec![];
        } else {
            group.push(answers.clone());
        }
    }
    groups.push(group.clone());

    let after0 = Instant::now();

    let start1 = Instant::now();

    let sum1: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .skip(1)
                .fold(group.first().unwrap().clone(), |gs, set| {
                    gs.union(set).cloned().collect()
                })
                .len()
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", sum1);
    }

    let start2 = Instant::now();

    let sum2: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .skip(1)
                .fold(group.first().unwrap().clone(), |gs, set| {
                    gs.intersection(set).cloned().collect()
                })
                .len()
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", sum2);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
