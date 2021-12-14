use super::tools;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_21_test.txt"
    } else {
        "./input/day14_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let polymer: Vec<char> = input[0].chars().collect();
    let last: char = polymer.iter().last().unwrap().clone();

    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    input.iter().skip(2).for_each(|line| {
        let mut tokens = line.split(" -> ");
        let from = tokens.next().unwrap().chars().collect::<Vec<char>>();
        let to: char = tokens.next().unwrap().chars().next().unwrap();
        rules.insert(from, to);
    });

    let count_letters = | counts: &HashMap<(char, char), usize> | -> HashMap<char, usize> {
        let mut char_counts = counts.iter().map(|c| (c.0 .0, c.1)).fold(
            HashMap::new(), |mut map: HashMap<char, usize>, (c, count)| { *map.entry(c).or_insert(0) += *count; map }
        );
        *char_counts.entry(last).or_insert(0) += 1;
        char_counts    
    };

    let step = | counts: &HashMap<(char, char), usize> | -> HashMap<(char, char), usize> {
        rules
            .iter()
            .fold(HashMap::new(), |mut newcounts, (pair, &newc)| {
                if let Some(cc) = counts.get(&(pair[0], pair[1])) {
                    *newcounts.entry((pair[0], newc)).or_insert(0) += cc;
                    *newcounts.entry((newc, pair[1])).or_insert(0) += cc;
                }
                newcounts
            })
    };

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut counts = polymer.windows(2).fold(HashMap::new(), |mut map, p| {
        *map.entry((p[0],p[1])).or_insert(0) += 1; map
    });

    (0..10).for_each(|_| { counts = step(&counts); });

    let char_counts = count_letters(&counts);
    let res1 = char_counts.iter().map(|(_, c)| *c).max().unwrap()
        - char_counts.iter().map(|(_, c)| *c).min().unwrap();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    (10..40).for_each(|_| { counts = step(&counts); });

    let char_counts = count_letters(&counts);
    let res2 = char_counts.iter().map(|(_, c)| *c).max().unwrap()
        - char_counts.iter().map(|(_, c)| *c).min().unwrap();

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
