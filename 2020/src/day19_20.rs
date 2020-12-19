use super::tools;
use std::{cmp::Ordering, time::Instant};
use regex::Regex;
use std::collections::HashMap;

fn match_list(s: &String, b: usize, e: usize, current_rules: &Vec<usize>, 
        rules: &HashMap<usize, Vec<Vec<usize>>>, 
        ends: &HashMap<usize, String>, 
        dynproglist: &mut HashMap<(usize,usize,usize), bool>) -> bool {

    if b==e && current_rules.len()==0 { return true }
    if b==e { return false }
    if current_rules.len() == 0 { return false }

    let mut res = false;

    let mut rest_rules = current_rules.clone();
    rest_rules.remove(0);
    for i in b+1..e+1 {
        if match_line(s, b, i, current_rules[0], rules, ends, dynproglist) && 
            match_list(s, i, e, &rest_rules, rules, ends, dynproglist) {
            res = true;
        }
    }

    res
}

fn match_line(s: &String, b: usize, e: usize, current_rule: usize, 
        rules: &HashMap<usize, Vec<Vec<usize>>>, 
        ends: &HashMap<usize, String>, 
        dynproglist: &mut HashMap<(usize,usize,usize), bool>) -> bool {

    let mut res = false;

    let key = (b,e,current_rule);
    if dynproglist.contains_key(&key) {
        return dynproglist[&key];
    }

    if ends.contains_key(&current_rule) {
        res = s[b..e].cmp(ends[&current_rule].as_str()) == Ordering::Equal;
    } else {
        for option in &rules[&current_rule] {
            if match_list(s, b, e, option, rules, ends, dynproglist) {
                res = true;
            }
        }
    }

    dynproglist.insert(key, res);

    res
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day19_20_test.txt"
    } else {
        "./input/day19_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut rules: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut ends: HashMap<usize, String> = HashMap::new();
    let mut tobechecked: Vec<String> = vec![];

    let mut dynprog: HashMap<(usize,usize,usize), bool> = HashMap::new();

    let rules_regex = Regex::new(r#"([0-9]+): (["\|a-z0-9 ]+)"#).expect("Invalid regex");
    let numbers = Regex::new(r"[0-9]+").expect("Invalid finish regex");


    for line in &input {
        if rules_regex.is_match(line) { 
            let captures = rules_regex.captures_iter(line).next().unwrap();
            let rule_id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let temprule_str = captures.get(2).unwrap().as_str();
            let rule_str = temprule_str.replace("\"", "");

            if numbers.is_match(rule_str.as_str()) {
                let mut full_options: Vec<Vec<usize>> = vec![];
                let options: Vec<String> = rule_str.split(" | ").map(|s| String::from(s)).collect();
                for option in &options {
                    let ands: Vec<usize> = option.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
                    full_options.push(ands);
                }
                rules.insert(rule_id, full_options.clone());
            } else {
                ends.insert(rule_id, rule_str);
            }

        } else {
            if line.len() > 0 {
                tobechecked.push(line.clone());
            }
        }
    }

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = tobechecked
        .iter()
        .map(|line| {
            dynprog.clear();
            return match_line(line, 0, line.len(), 0, &rules, &ends, &mut dynprog) 
        })
        .filter(|&v| v )
        .count();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    if let Some(r) = rules.get_mut(&8) {
        *r = vec![vec![42], vec![42, 8]];
    }

    if let Some(r) = rules.get_mut(&11) {
        *r = vec![vec![42, 31], vec![42, 11, 31]];
    }

    let res2 = tobechecked
        .iter()
        .map(|line| {
            dynprog.clear();
            return match_line(line, 0, line.len(), 0, &rules, &ends, &mut dynprog) 
        })
        .filter(|&v| v )
        .count();


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
