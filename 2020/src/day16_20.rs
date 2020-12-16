use super::tools;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day16_20_test.txt"
    } else {
        "./input/day16_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut rules: Vec<((usize,usize),(usize,usize))> = vec![];
    let mut valid: Vec<Vec<bool>>;
    let mut tickets: Vec<Vec<usize>> = vec![];
    let mut myticket = vec![];
    let mut rulenames: Vec<&str> = vec![];

    let mut first: bool = true;

    let rules_regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").expect("Invalid regex");
    for line in &input {
        if rules_regex.is_match(line) {
            let captures = rules_regex.captures_iter(line).next().unwrap();
            let rule_name = captures.get(1).unwrap().as_str();
            rulenames.push(rule_name);
            let range1 = (captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),captures.get(3).unwrap().as_str().parse::<usize>().unwrap());
            let range2 = (captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),captures.get(5).unwrap().as_str().parse::<usize>().unwrap());
            rules.push((range1,range2));
        } else if line.contains(",") { // ticket
            let ticket: Vec<usize> = line.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
            if first {
                myticket = ticket.clone();
                first = false;
            } else {
                tickets.push(ticket.clone());
            }
        }
    }
    valid = vec![vec![true; rules.len()]; rules.len()];

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut error_rate = 0;
    let mut valid_tickets: Vec<Vec<usize>> = vec![];
    for t in &tickets {
        let mut is_valid = true;
        for &v in t {
            let mut count_valid = 0;
            for rule in &rules {
                if (rule.0.0 <= v && v <= rule.0.1) || (rule.1.0 <= v && v <= rule.1.1) { // valid rules
                    count_valid += 1;
                }
            }
            if count_valid == 0 {
                is_valid = false;
                error_rate += v;
            }
        }
        if is_valid {
            valid_tickets.push(t.clone());
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", error_rate);
    }

    let start2 = Instant::now();

    for (i, &rule) in rules.iter().enumerate() {
        for t in &valid_tickets {
            for (j, &v) in t.iter().enumerate() {
                if !((rule.0.0 <= v && v <= rule.0.1) || (rule.1.0 <= v && v <= rule.1.1)) { // invalid
                    valid[i][j] = false;
                } else {
                }
            }  
        }
    }

    let mut assigned: HashMap<usize,usize> = HashMap::new();

    for _ in 0..rules.len() {
        for i in 0..rules.len() {
            let mut truecount = 0;
            let mut single_index = rules.len()+1;
            for (index2,r) in valid.iter().enumerate() {
                if r.len() > 0 {
                    if r[i] { truecount += 1; single_index = index2; }
                }
            }
            if truecount == 1 {
                assigned.insert(i, single_index);
            }
        }

        for (_,&k) in &assigned {
            valid[k] = vec![];
        }
    }

    let mut res2: u64 = 1;
    for ass in &assigned {
        let name = rulenames[*ass.1];
        if name.contains("departure") {
            res2 *= myticket[*ass.0] as u64;
        }
    }

    // println!("{:?}", valid);

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
