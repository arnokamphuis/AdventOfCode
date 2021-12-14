use super::tools;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::BTreeSet;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day14_21_test.txt"
    } else {
        "./input/day14_21_real.txt"
    };
    let input = tools::get_input(String::from(input_file));

    let mut polymer: Vec<char> = input[0].chars().collect();
    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    let mut chars: BTreeSet<char> = BTreeSet::new();
    println!("polymer: {:?}", polymer);
    input.iter().skip(2).for_each(|line| {
        let mut tokens = line.split(" -> ");
        let from = tokens.next().unwrap().chars().collect::<Vec<char>>();
        let to: char = tokens.next().unwrap().chars().next().unwrap();
        chars.insert(from[0]); chars.insert(from[1]); chars.insert(to);
        rules.insert(from,to);
    });

    let after0 = Instant::now();

    let start1 = Instant::now();

    (0..10).for_each(|_| {
        let last:char = polymer.iter().last().unwrap().clone();
        polymer = polymer.windows(2).fold(vec![], |mut poly, base| {
            poly.push(base[0]);
            poly.push(rules[base]);
            // poly.push(base[1]);
            poly
        });
        polymer.push(last);


        let counts = chars.iter().map(|c| {
            polymer.iter().filter(|ch| *ch==c).count()
        }).collect::<Vec<usize>>();
        // println!("{:?}", counts);
        // println!("counts: {:?}", counts.iter().max().unwrap() - counts.iter().min().unwrap());

    });
    // println!("{:?}", polymer);

    let counts = chars.iter().map(|c| {
        polymer.iter().filter(|ch| *ch==c).count()
    }).collect::<Vec<usize>>();
    // println!("counts: {:?}", counts);
    // println!("counts: {:?}", counts.iter().max().unwrap() - counts.iter().min().unwrap());
    
    let res1 = counts.iter().max().unwrap() - counts.iter().min().unwrap();
    
    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    // (10..40).for_each(|_| {
    //     let last:char = polymer.iter().last().unwrap().clone();
    //     polymer = polymer.windows(2).fold(vec![], |mut poly, base| {
    //         poly.push(base[0]);
    //         poly.push(rules[base]);
    //         // poly.push(base[1]);
    //         poly
    //     });
    //     polymer.push(last);
    // });
    // // println!("{:?}", polymer);

    // let counts = chars.iter().map(|c| {
    //     polymer.iter().filter(|ch| *ch==c).count()
    // }).collect::<Vec<usize>>();
    
    // println!("counts: {:?}", counts);
    // let res2 = counts.iter().max().unwrap() - counts.iter().min().unwrap();
    let res2 = 0;

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
