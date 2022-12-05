use super::tools;
use std::time::Instant;

#[allow(dead_code)]
pub fn run(real: bool, print_result: bool) -> (u128, u128, u128) {
    let start0 = Instant::now();

    let input_file: &str = if !real {
        "./input/day05_22_test.txt"
    } else {
        "./input/day05_22_real.txt"
    };
    let input = tools::get_input(String::from(input_file));
    let n = if !real { 3 } else { 9 };
    let emptyline = input.iter().enumerate().filter(|(_,line)| line.len()==0).map(|(index,_)| index).collect::<Vec<usize>>()[0];

    let mut piles: Vec<Vec<char>> = vec![vec![];n];
    input[..emptyline-1].iter().for_each(|line| { 
        for i in 0..n {
            let c = line.chars().nth(4*i+1).unwrap();
            if c != ' ' {
                piles[i].insert(0,c);
            }
        }
    });
    let original_piles = piles.clone();

    let moves = input[emptyline+1..].iter().map(|line| { 
        line
            .splitn(6, [' '])
            .filter(|part| !part.contains("o"))
            .map(|s| s.to_string())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }).collect::<Vec<_>>();

    let after0 = Instant::now();

    let start1 = Instant::now();

    moves
        .iter()
        .for_each(|moveit| {
            let amount = moveit[0];
            let from   = moveit[1]-1;
            let to     = moveit[2]-1;

            let f_n = piles[from].len()-1;
            (0..amount).for_each(|i| {
                let c = piles[from].remove(f_n-i);
                piles[to].push(c);
            });
        });

    let res1 = piles.iter().map(|pile| {
        let index = pile.len()-1;
        pile[index]
    }).collect::<String>();

    let after1 = Instant::now();
    if print_result {
        println!("Part 1: {}", res1);
    }

    let start2 = Instant::now();

    piles = original_piles.clone();
    moves
        .iter()
        .for_each(|moveit| {
            let amount = moveit[0];
            let from   = moveit[1]-1;
            let to     = moveit[2]-1;

            let f_n = piles[from].len()-1;
            (0..amount).for_each(|_| {
                let c = piles[from].remove(f_n-amount+1);
                piles[to].push(c);
            });
        });

    let res2 = piles.iter().map(|pile| {
        let index = pile.len()-1;
        pile[index]
    }).collect::<String>();

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
