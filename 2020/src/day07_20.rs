use super::tools;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use queues::*;

pub fn count_bags(current_bag: &String, factor: u128, rules: &BTreeMap<String, Vec<(usize, String)>>) -> u128 {
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

    let input_file: &str = if !real { "./input/day07_20_test.txt" } else { "./input/day07_20_real.txt" };
    let input = tools::get_input(String::from(input_file));

    let mut rules: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();

    for line in &input {
        let mut rule: Vec<(usize, String)> = vec![];
        let words: Vec<&str> = line.split(' ').collect();
        let mut w_iter = words.iter().peekable();

        let mut bag = w_iter.next().unwrap().to_string();
        bag.push_str(w_iter.next().unwrap());
        bag.push_str(w_iter.next().unwrap());
        bag = String::from(&bag[0..bag.len()-1]);
        w_iter.next();

        while w_iter.peek() != None {
            let amount_str = w_iter.next().unwrap();
            if amount_str.to_string() == "no" {
                w_iter.next(); w_iter.next();
            } else {
                let amount = amount_str.parse::<usize>().unwrap();
                let mut otherbag = w_iter.next().unwrap().to_string();
                otherbag.push_str(w_iter.next().unwrap());
                otherbag.push_str(w_iter.next().unwrap());
                if amount > 1 {
                    otherbag = String::from(&otherbag[0..otherbag.len()-2]);
                } else {
                    otherbag = String::from(&otherbag[0..otherbag.len()-1]);
                }
                rule.push( (amount, otherbag) );
            }
        }
        rules.insert(bag, rule.clone());
    }

    let target = String::from("shinygoldbag");

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
    if print_result { println!("Part 1: {}", options.len()); }

    let start2 = Instant::now();

    let total = count_bags(&target, 1, &rules)-1; // subtract one because target is counted

    let after2 = Instant::now();
    if print_result { println!("Part 2: {}", total); }

    (after0.duration_since(start0).as_nanos(), after1.duration_since(start1).as_nanos(), after2.duration_since(start2).as_nanos())
}
