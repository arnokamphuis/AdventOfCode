use super::tools;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;
use std::collections::BTreeSet;

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

    let check_rule = |ticket_value: usize, rule: ((usize,usize),(usize,usize))| -> bool {
        (rule.0.0 <= ticket_value && ticket_value <= rule.0.1) || (rule.1.0 <= ticket_value && ticket_value <= rule.1.1)
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1: usize = tickets
        .iter()
        .fold(0, |error_rate, ticket| 
            error_rate + ticket.iter().filter(|&&ticket_value| 
                rules
                    .iter()
                    .filter(|&&rule | check_rule(ticket_value, rule))
                    .count() == 0 
            )
            .sum::<usize>()
        );

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let valid_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .filter(|&ticket| {
            ticket.iter().filter(|&&ticket_value| 
                rules
                    .iter()
                    .filter(|&&rule | check_rule(ticket_value, rule))
                    .count() == 0 
            )
            .sum::<usize>() == 0
        }).cloned().collect::<Vec<Vec<usize>>>();


    let mut sets: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); rules.len()];
    sets.iter_mut().for_each(|set| {
        (0..rules.len()).for_each(|id| { set.insert(id); });
    });

    rules.iter().enumerate().for_each(|(rule_index, &rule)| {
        valid_tickets.iter().for_each(|ticket| {
            ticket.iter().enumerate().for_each(|(ticket_value_index, &ticket_value)| 
                if !check_rule(ticket_value, rule) { // invalid
                    if let Some(mut_set) = sets.get_mut(rule_index) {
                        mut_set.remove(&ticket_value_index);
                    }
                }
            );
        });
    });

    // build map for each rule to ticket value
    let mut assigned: HashMap<usize,usize> = HashMap::new();

    // filter all sets based on already assigned rule/item pairs
    while sets.iter().fold(0, |acc, v| acc + v.len() ) > 0 {
        sets.iter().enumerate()
            .filter(|(_, set)| set.len() == 1)
            .for_each(|(index1, aset)| {
                assigned.insert(index1, aset.iter().fold(0, |acc, v| acc+v));
            });

        assigned.iter().for_each(|(_, ass_to)| {
            sets.iter_mut().for_each(|set| { set.remove(ass_to); });
        });
    } 

    let res2: u64 = rulenames
        .iter()
        .enumerate()
        .filter(|(_, &name)| name.contains("departure"))
        .map(|(index, _)| myticket[*(assigned.get(&index).unwrap())] as u64)
        .product::<u64>();

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
