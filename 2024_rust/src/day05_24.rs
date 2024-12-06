use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::Ordering;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05-test.txt"
    } else {
        "./input/day05-real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let mut cache: HashMap<(u16,u16), Ordering> = HashMap::new();
    let mut pages: Vec<Vec<u16>> = Vec::new();
    let mut break_line = 0;
    input.iter().enumerate().for_each(|(i, line)| {
        if line == ""{
            break_line = i;
            return;
        }
        if break_line == 0 {
            let Some((l,r)) = line
                .split("|")
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple() else { todo!() };
            cache.insert((l,r), Ordering::Less);
            cache.insert((r,l), Ordering::Greater);
        } else {
            pages.push(line.split(",").map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>())
        }
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let res1 = pages
        .iter()
        .filter(|page| {
            page.iter().sorted_by(|&a, &b| cache[&(*a,*b)]).map(|&x|x).collect::<Vec<u16>>() == **page
        })
        .map(|page| page[page.len()/2 as usize])
        .sum::<u16>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = pages
        .iter()
        .filter(|page| {
            page.iter().sorted_by(|&a, &b| cache[&(*a,*b)]).map(|&x|x).collect::<Vec<u16>>() != **page
        })
        .map(|page| {
            page.iter().sorted_by(|&a, &b| cache[&(*a,*b)]).map(|&x|x).collect::<Vec<u16>>()
        })
        .map(|page| page[page.len()/2 as usize])
        .sum::<u16>();

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
