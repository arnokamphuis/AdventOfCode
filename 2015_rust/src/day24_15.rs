use super::tools;
use std::time::Instant;
use std::collections::{BTreeSet,BTreeMap};

fn find_group_with_sum(target: usize, numbers: &Vec<usize>, partial: &Vec<usize>, sets: &mut BTreeSet<Vec<usize>>) {
    let partial_sum: usize = partial.iter().sum::<usize>();

    if partial_sum == target {
        sets.insert(partial.clone());
        return
    }
    if partial_sum > target {
        return
    }

    for i in 0..numbers.len() {
        let mut next_partial = partial.clone();
        next_partial.push(numbers[i]);

        let remaining = numbers[i+1..].to_vec();
        find_group_with_sum(target, &remaining, &next_partial, sets);
    }
}


#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day24_15_test.txt"
    } else {
        "./input/day24_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let weights = input.iter().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    
    let after0 = Instant::now();

    let start1 = Instant::now();

    let find_minimum = | number: usize, ws: &Vec<usize> | -> usize {
        let total_weights: usize = ws.iter().sum();
        let target_weight = total_weights / number;
        let mut sets: BTreeSet<Vec<usize>> = BTreeSet::new();
    
        find_group_with_sum(target_weight, &ws, &vec![], &mut sets);
    
        let mut map: BTreeMap<usize, BTreeSet<Vec<usize>>> = BTreeMap::new();
        for set in &sets {
            map.entry(set.len()).and_modify(|s| {s.insert(set.clone());}).or_insert({
                let mut new_set = BTreeSet::new();
                new_set.insert(set.clone());
                new_set
            });
        }
    
        let smallest = map.iter().map(|(&s,_)| s).min().unwrap();
    
        map[&smallest].iter().map(|v| v.iter().fold(1, |prod, n| prod * n)).min().unwrap()
    };

    let res1 = find_minimum( 3, &weights ); 

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = find_minimum( 4, &weights ); 

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
