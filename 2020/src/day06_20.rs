use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool) {
    println!("Day 06 of 2020");

    let start0 = Instant::now();

    let input_file: &str = if !real { "./input/day06_20_test.txt" } else { "./input/day06_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let mut groups: Vec<Vec<BTreeSet<char>>> = vec![];
    let mut group: Vec<BTreeSet<char>> = vec![];

    for line in &input {
        let mut answers: BTreeSet<char> = BTreeSet::new();
        for c in line.chars() {
            answers.insert(c);
        }

        if answers.len() == 0 {
            groups.push(group.clone());
            group = vec![];
        } else {
            group.push(answers.clone());
        }
    }
    groups.push(group.clone());

    let after0 = Instant::now();
    println!("Init in {:?}", after0.duration_since(start0));

    let start1 = Instant::now();

    let sum1: usize = groups.iter().map(|group|
        group.iter().skip(1).fold(group.first().unwrap().clone(), |gs, set| gs.union(set).cloned().collect() ).len()
    ).collect::<Vec<usize>>().iter().sum();

    let after1 = Instant::now();
    println!("Part 1: {}, in {:?}", sum1, after1.duration_since(start1));

    let start2 = Instant::now();

    let sum2: usize = groups.iter().map(|group|
        group.iter().skip(1).fold(group.first().unwrap().clone(), |gs, set| gs.intersection(set).cloned().collect() ).len()
    ).collect::<Vec<usize>>().iter().sum();

    let after2 = Instant::now();
    println!("Part 2: {}, in {:?}", sum2, after2.duration_since(start2));
}
