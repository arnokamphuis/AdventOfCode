use super::tools;
use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day16_15_test.txt"
    } else {
        "./input/day16_15_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let sues = input.iter().enumerate().map(|(id,line)| {
        (id+1, line
            .split(" ")
            .collect::<Vec<&str>>()[2..].iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|arr| (arr[0].to_string().replace(":",""), arr[1].to_string().replace(",","").parse::<usize>().unwrap()))
            .collect::<HashMap<String,usize>>()
        )
    }).collect::<HashMap<usize,_>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    let profile: HashMap<&str, usize> = vec![
        ("children", 3), ("cats", 7), ("samoyeds", 2), ("pomeranians", 3),
        ("akitas", 0), ("vizslas", 0), ("goldfish", 5), ("trees", 3),
        ("cars", 2), ("perfumes", 1)
    ].into_iter().collect();

    let res1 = sues.iter()
        .filter(|(_, sue)| {
            sue.iter().map(|(n,v)| profile.get(n.as_str()).unwrap() == v ).fold(true, |truth, b| truth && b)
        })
        .map(|(&id, _)| id)
        .collect::<Vec<usize>>()[0];

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let res2 = sues.iter()
        .filter(|(_, sue)| {
            sue.iter().map(|(n,v)| match n.as_str() {
                "cats" | "trees" => profile.get(n.as_str()).unwrap() < v, 
                "pomeranians" | "goldfish" => profile.get(n.as_str()).unwrap() > v, 
                _ => profile.get(n.as_str()).unwrap() == v, 
            } ).fold(true, |truth, b| truth && b)
        })
        .map(|(&id, _)| id)
        .collect::<Vec<usize>>()[0];

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
