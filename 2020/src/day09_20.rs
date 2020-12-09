use super::tools;
use std::time::Instant;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day09_20_test.txt"
    } else {
        "./input/day09_20_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let window: usize = if real { 25 } else {5};
    let numbers: Vec<i64> = input.iter().map(|v| v.parse::<i64>().unwrap()).collect();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut contiguoustarget: i64 = 0;

    for i in window..numbers.len() {
        let target = numbers.iter().nth(i).unwrap();
        let sources = numbers.iter().skip(i-window).take(window).collect::<Vec<&i64>>();

        let mut found = false;
        for j in 0..sources.len() {
            for k in 0..sources.len() {
                if j!=k {
                    if sources[j] + sources[k] == *target {
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            contiguoustarget = numbers[i];
            break;
        }
    }

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", contiguoustarget);
    }

    let start2 = Instant::now();


    let mut seen: BTreeSet<Vec<i64>> = BTreeSet::new();
    let mut res2 = 0;
    if let Some(res) = filter_to_target(numbers.iter().map(|v| v).collect::<Vec<&i64>>(), contiguoustarget,contiguoustarget, vec![], &mut seen) {
        // println!("{:?}",res);
        res2 = res.iter().min().unwrap()+res.iter().max().unwrap();
    }

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

pub fn filter_to_target(set: Vec<&i64>, final_target: i64, target: i64, contains: Vec<i64>, seen: &mut BTreeSet<Vec<i64>> ) -> Option<Vec<i64>> {
    // println!("     FILTER TO TARGET: {:?}, {}, {:?}", set,target,contains);
    let smaller_set = set.iter().filter(|v| **v < &target ).map(|v| *v).collect::<Vec<&i64>>();
    if smaller_set.len() == 0 {
        return None;
    }

    let uptonow = contains.iter().sum::<i64>();
    if contains.len() > 1 && set.iter().filter(|&&&v| uptonow+v==final_target).count() == 1 {
        let mut new_contains = contains.clone();
        new_contains.push(set.iter().filter(|&&&v| uptonow+v==final_target).fold(0, |acc, &&v| acc+v));
        // println!("YES");
        return Some(new_contains);
    }

    let small_diff_set: Vec<(&i64,i64)> = smaller_set.clone()
        .iter()
        .map(|v| (*v, target-**v) )
        .collect();

    // println!("--------------------------------------------------------------------------");
    // println!("{:?}", small_diff_set);
    for (s,t) in small_diff_set.iter() {
        let new_target = *t;
        let set: Vec<&i64> = smaller_set.iter().filter(|v|  **s != ***v ).map(|v| *v).collect();//***v < new_target &&
        let mut new_contains = contains.clone();
        new_contains.push(**s);

        let mut pot_seen = new_contains.clone();
        pot_seen.sort();
        if seen.contains(&pot_seen) {

        } else {
            seen.insert(pot_seen);
        
            // println!("{} {} open: {:?}  close: {:?}", s, new_target, set, new_contains);

            if let Some(res) = filter_to_target(set, final_target, new_target, new_contains, seen) {
                return Some(res);
            }
        }
    }
    // println!("--------------------------------------------------------------------------");
    return None
}