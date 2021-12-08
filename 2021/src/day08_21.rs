use super::tools;
use permutohedron::heap_recursive;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day08_21_test.txt"
    } else {
        "./input/day08_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut digits: HashMap<usize, BTreeSet<char>> = HashMap::new();
    digits.insert(0, BTreeSet::from(['a', 'b', 'c', 'e', 'f', 'g']));
    digits.insert(1, BTreeSet::from(['c', 'f']));
    digits.insert(2, BTreeSet::from(['a', 'c', 'd', 'e', 'g']));
    digits.insert(3, BTreeSet::from(['a', 'c', 'd', 'f', 'g']));
    digits.insert(4, BTreeSet::from(['b', 'c', 'd', 'f']));
    digits.insert(5, BTreeSet::from(['a', 'b', 'd', 'f', 'g']));
    digits.insert(6, BTreeSet::from(['a', 'b', 'd', 'e', 'f', 'g']));
    digits.insert(7, BTreeSet::from(['a', 'c', 'f']));
    digits.insert(8, BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    digits.insert(9, BTreeSet::from(['a', 'b', 'c', 'd', 'f', 'g']));

    let reverse_digits = digits.iter().fold(HashMap::new(), |mut m, (n,s)| {m.insert(s,n); m});

    let mut firsts: Vec<Vec<BTreeSet<char>>> = vec![];
    let mut seconds: Vec<Vec<BTreeSet<char>>> = vec![];
    input.iter().for_each(|line| {
        let twoparts: Vec<String> = line.split(" | ").fold(vec![], |mut list: Vec<String>, s| {
            list.push(String::from(s));
            list
        });
        let first: Vec<BTreeSet<char>> =
            twoparts[0]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        firsts.push(first);
        let second: Vec<BTreeSet<char>> =
            twoparts[1]
                .split_whitespace()
                .fold(vec![], |mut list: Vec<BTreeSet<char>>, s| {
                    list.push(s.chars().collect());
                    list
                });
        seconds.push(second);
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    let mut res1 = 0;
    seconds.iter().for_each(|second| {
        let digits = second
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .map(|v| v.clone())
            .collect::<Vec<BTreeSet<char>>>();
        res1 += digits.iter().count();
    });

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    let mut permutations = vec![];
    let mut data = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });

    let res2:usize = firsts
        .iter()
        .zip(seconds.iter())
        .map(|(first, second)| {
            let mut found: Vec<char> = vec![];
            'outer: for permutation in &permutations {
                
                let translation = data.iter().zip(permutation.iter()).fold(HashMap::new(), |mut hm, (&f,&t)| {hm.insert(f,t); hm});

                let mut ok = true;
                for (_,d) in &digits {
                    let newd = d.iter().map(|c| translation[c]).collect::<BTreeSet<_>>();
                    if !first.contains(&newd) {
                        ok = false;
                    }
                }
                if ok {
                    found = permutation.to_vec();
                    break 'outer;
                }

            }

            let translation = found.iter().zip(data.iter()).fold(HashMap::new(), |mut hm, (&f,&t)| {hm.insert(f,t); hm});

            second.iter().fold(0, |value, s| {
                let news = s.iter().map(|d| translation[d]).collect::<BTreeSet<_>>();
                value * 10 + reverse_digits[&news]
            })
        }).sum();

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
