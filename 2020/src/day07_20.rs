use super::tools;
use queues::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::time::Instant;
use regex::Regex;

pub fn count_bags(
    current_bag: &String,
    factor: u128,
    rules: &BTreeMap<String, Vec<(usize, String)>>,
) -> u128 {
    let mut amount = factor;

    if let Some(active_rule) = rules.get(current_bag) {
        for t in active_rule {
            amount += count_bags(&t.1, factor * (t.0 as u128), rules);
        }
    }

    amount
}

#[allow(dead_code)]
#[allow(unused_must_use)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day07_20_test.txt"
    } else {
        "./input/day07_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut rules: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();

    let from_regex = Regex::new(r"([\w\s]+) bags").expect("Invalid regex");
    let target_regex = Regex::new(r"(\d+) ([\w\s]+) bags?").expect("Invalid regex");

    for line in &input {
        let mut rule: Vec<(usize, String)> = vec![];

        let parts: Vec<&str> = line.split(" contain ").into_iter().collect();

        let from: Vec<_> = from_regex.captures_iter(parts[0]).collect();
        let from_bag = String::from(from[0].get(1).unwrap().as_str());

        for s in target_regex.captures_iter(parts[1]) {
            let amount = s.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let target = String::from(s.get(2).unwrap().as_str());
            rule.push((amount, target.clone()));
        }
        
        rules.insert(from_bag, rule.clone());
    }

    let target = String::from("shiny gold");

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut options: BTreeSet<String> = BTreeSet::new();
    let mut q: Queue<String> = queue![target.clone()];

    while let Ok(bag) = q.remove() {
        for rule in &rules {
            for t in rule.1 {
                if t.1 == bag {
                    options.insert(rule.0.clone());
                    q.add(rule.0.clone());
                }
            }
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", options.len());
    }

    let start2 = Instant::now();

    let total = count_bags(&target, 1, &rules) - 1; // subtract one because target is counted

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {}", total);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
