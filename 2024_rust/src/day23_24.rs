use super::tools;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;


fn create_scc<'a>(computers: HashSet<&'a str>, connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> Option<HashSet<&'a str>> {
    let mut sets: Vec<&HashSet<&str>> = Vec::new();
    for comp in computers.iter() {
        sets.push(connections.get(comp).unwrap());
    }
    let targets: HashSet<&str> = sets[1..].iter().fold(sets[0].clone(), |inter, set| {
        inter.intersection(set).map(|s| *s).collect()
    });

    if targets.len() == 0 {
        return Some(computers.clone());
    }

    for t in targets.iter() {
        let mut new_computers = computers.clone();
        new_computers.insert(t);
        if let Some(result) = create_scc(new_computers, connections) {
            return Some(result.clone());
        }
    }
    None
}

fn find_all_sccs<'a>(computers: &'a Vec<&'a str>, connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut ccs: Vec<Vec<&str>> = Vec::new();
    for c1 in computers.iter() {
        for c2 in connections.get(c1).unwrap().iter() {
            let start_set = [*c1, *c2].iter().map(|s|*s).collect::<HashSet<&str>>();
            if let Some(cc) = create_scc(start_set, connections) {
                let sccs = cc.iter().sorted().map(|s| *s).collect::<Vec<&str>>();
                if !ccs.contains(&sccs) {
                    ccs.push(sccs);
                }
            }
        }
    }
    ccs
}

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day23-test.txt"
    } else {
        "./input/day23-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let connections: HashMap<&str, HashSet<&str>> = input.iter().fold(HashMap::new(), |mut map, line| {
        let parts: Vec<&str> = line.split("-").collect();
        let f = parts[0];
        let t = parts[1];
        map.entry(f).or_insert(HashSet::new()).insert(t);
        map.entry(t).or_insert(HashSet::new()).insert(f);
        map
    });

    let computers = connections.keys().map(|s| *s).collect::<Vec<&str>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut triples: HashSet<Vec<&str>> = HashSet::new();
    for c1 in computers.iter() {
        for c2 in connections.get(c1).unwrap().iter() {
            for c3 in connections.get(c2).unwrap().iter() {
                if connections.get(c3).unwrap().contains(c1) {
                    triples.insert(vec![*c1, *c2, *c3].iter().sorted().map(|s| *s).collect());
                }
            }
        }
    }
    let res1 = triples.iter().filter(|t| {
        t.iter().any(|c| c.starts_with("t"))
    }).count();
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let connected_components = find_all_sccs(&computers, &connections);
    let max_cc = connected_components.iter().map(|cc| cc.len()).max().unwrap();
    let max_connected_comp = connected_components
        .iter()
        .filter(|cc| { cc.len() == max_cc })
        .collect::<Vec<&Vec<&str>>>()[0]
        .iter()
        .fold("".to_string(), |acc, cc| { acc + "," + cc })[1..]
        .to_string();

    let after2 = Instant::now();
    if print_result {
        println!("Part 2: {:?}", max_connected_comp);
    }

    (
        after0.duration_since(start0).as_nanos(),
        after1.duration_since(start1).as_nanos(),
        after2.duration_since(start2).as_nanos(),
    )
}
